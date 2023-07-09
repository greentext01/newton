use std::{
    net::SocketAddr,
    sync::{mpsc::Sender, Arc, RwLock},
};

use common::{data::state::State, messages::from_server::FromServerMessage};
use message_io::{
    network::{Endpoint, NetEvent, Transport},
    node::{self, NodeEvent, NodeHandler, NodeListener},
};

/// NetThreadEvents are events that are sent to the graphics thread.
/// These are sent by a mpsc channel.
pub enum NetThreadEvent {
    Quit,
    Connected,
    Disconnected,
}

/// Signals are message-io's way of giving the network thread a way of telling
/// itself to perform some action, such as quitting.
///
/// More info: https://docs.rs/message-io/0.5.0/message_io/node/struct.NodeSignals.html
enum Signal {
    Quit,
}

/// This is used to configure the network options of the client.
/// Other options may be added in the future.
pub struct Config {
    pub network_interface: &'static str,
    pub network_port: u16,
}

/// The client is responsible for connecting to the server, and receiving updates from it.
/// The client is also responsible for sending inputs to the server.
///
/// This is all done through message-io, a library which provides easy networking.
pub struct Client {
    node: NodeHandler<Signal>,
    listener: NodeListener<Signal>,
    server_id: Endpoint,
    local_addr: SocketAddr,
    state_lock: Arc<RwLock<Option<State>>>,
    events_tx: Sender<NetThreadEvent>,
}

impl Client {
    /// Creates a new client, based on the given configuration.
    ///
    /// Args:
    pub fn new(
        config: Config,
        state_lock: Arc<RwLock<Option<State>>>,
        events_tx: Sender<NetThreadEvent>,
    ) -> Option<Client> {
        let (node, listener) = node::split();
        let connection_result = node.network().connect(
            Transport::FramedTcp,
            (config.network_interface, config.network_port),
        );

        let (server_id, local_addr) = match connection_result {
            Err(_) => {
                log::error!(
                    "Failed to listen on {}:{}",
                    config.network_interface,
                    config.network_port
                );
                return None;
            }

            Ok(res) => res,
        };

        log::info!(
            "Client is listening on {}:{}",
            config.network_interface,
            config.network_port
        );

        Some(Client {
            node,
            listener,
            server_id,
            local_addr,
            state_lock,
            events_tx,
        })
    }

    pub fn run(self) {
        let node_closer = self.node.clone();

        let ctrlc_handler_res = ctrlc::set_handler(move || {
            node_closer.signals().send(Signal::Quit);
        });

        if let Err(_) = ctrlc_handler_res {
            log::error!("Error setting Ctrl-C handler");
        }

        self.listener.for_each(move |message| match message {
            NodeEvent::Network(net_event) => match net_event {
                NetEvent::Connected(_, established) => {
                    if established {
                        log::info!("Connected to server at {}", self.server_id.addr());
                        log::info!(
                            "Client identified by local port: {}",
                            self.local_addr.port()
                        );

                        if self.events_tx.send(NetThreadEvent::Connected).is_err() {
                            log::error!("Failed to send connected event to graphics thread.\nThe receiver was somehow dropped.");
                        }
                    } else {
                        log::error!("Cannot connect to server at {}", self.server_id.addr());
                        self.node.signals().send_with_priority(Signal::Quit);
                    }
                }
                NetEvent::Accepted(_, _) => {}
                NetEvent::Message(_, message_bin) => {
                    let message: FromServerMessage = bincode::deserialize(&message_bin).unwrap();
                    match message {
                        FromServerMessage::Update(state) => {
                            log::trace!("Received state update");
                            let mut state_guard = self.state_lock.write().unwrap();
                            *state_guard = Some(state);
                        }
                    }
                }
                NetEvent::Disconnected(_) => {
                    log::info!("Disconnected from server. Stopping...");

                    if self.events_tx.send(NetThreadEvent::Disconnected).is_err() {
                        log::error!("Failed to send disconnected event to graphics thread.\nThe receiver was somehow dropped.");
                    }

                    self.node.signals().send_with_priority(Signal::Quit);
                }
            },
            NodeEvent::Signal(signal) => match signal {
                Signal::Quit => {
                    log::trace!("Stopping network thread...");
                    self.node.stop();

                    log::trace!("Setting quit flag...");
                    if self.events_tx.send(NetThreadEvent::Quit).is_err() {
                        log::error!("Failed to send quit event to graphics thread.\nThe receiver was somehow dropped.");
                    }
                }
            },
        });
    }
}
