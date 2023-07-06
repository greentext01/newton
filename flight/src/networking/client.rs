use std::{
    net::SocketAddr,
    sync::{atomic::{AtomicBool, Ordering}, Arc, RwLock},
};

use common::{data::state::State, messages::from_server::FromServerMessage};
use message_io::{
    network::{Endpoint, Transport, NetEvent},
    node::{self, NodeHandler, NodeListener, NodeEvent},
};

enum Signal {
    Quit,
}

pub struct Config {
    pub network_interface: &'static str,
    pub network_port: u16,
}

pub struct Client {
    node: NodeHandler<Signal>,
    listener: NodeListener<Signal>,
    server_id: Endpoint,
    local_addr: SocketAddr,
    state_lock: Arc<RwLock<Option<State>>>,
    quit_flag: Arc<AtomicBool>,
}

impl Client {
    pub fn new(
        config: Config,
        state_lock: Arc<RwLock<Option<State>>>,
        quit_flag: Arc<AtomicBool>,
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
            quit_flag,
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
                        log::info!("Client identified by local port: {}", self.local_addr.port());
                    } else {
                        log::error!("Cannot connect to server at {}", self.server_id.addr());
                        self.node.signals().send_with_priority(Signal::Quit);
                    }
                }
                NetEvent::Accepted(_, _) => {}
                NetEvent::Message(_, message_bin) => {
                    let message: FromServerMessage =
                        bincode::deserialize(&message_bin).unwrap();
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
                    self.node.signals().send_with_priority(Signal::Quit);
                }
            },
            NodeEvent::Signal(signal) => match signal {
                Signal::Quit => {
                    log::trace!("Stopping network thread...");
                    self.node.stop();

                    log::trace!("Setting quit flag...");
                    self.quit_flag.store(true, Ordering::SeqCst);
                }
            },
        });
    }
}
