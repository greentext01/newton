use std::collections::HashSet;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use common::data::inputs::Inputs;
use common::data::object::Objects;
use common::data::state::State;
use common::messages::encoding::BincodeEncoder;
use common::messages::from_client::FromClientMessage;
use common::messages::from_server::FromServerMessage;
use lazy_static::lazy_static;
use message_io::network::{Endpoint, NetEvent};
use message_io::node::{NodeEvent, NodeHandler};
use message_io::{
    network::Transport,
    node::{self, NodeListener},
};

lazy_static! {
    static ref UPDATE_DURATION: Duration = Duration::from_secs_f32(1.0 / 30.0);
}

#[derive(Debug)]
enum Signal {
    Update,
    Close,
}

pub struct Server {
    encoder: BincodeEncoder,
    objects_rwlock: Arc<RwLock<Objects>>,
    inputs_rwlock: Arc<RwLock<Inputs>>,
    listener: Option<NodeListener<Signal>>,
    subscriptions: HashSet<Endpoint>,
    node: NodeHandler<Signal>,
}

pub struct Config {
    pub network_interface: &'static str,
    pub network_port: u16,
}

impl Server {
    pub fn new(
        config: Config,
        objects_rwlock: Arc<RwLock<Objects>>,
        input_rwlock: Arc<RwLock<Inputs>>,
    ) -> Option<Server> {
        let (node, listener) = node::split();
        let node_closer = node.clone();

        let ctrlc_handler_res = ctrlc::set_handler(move || {
            node_closer.signals().send_with_priority(Signal::Close);
        });

        match ctrlc_handler_res {
            Err(message) => log::error!("Failed to set ctrlc handler: {}", message),
            _ => (),
        }

        if node
            .network()
            .listen(
                Transport::FramedTcp,
                (config.network_interface, config.network_port),
            )
            .is_err()
        {
            log::error!(
                "Failed to listen on {}:{}",
                config.network_interface,
                config.network_port
            );
            return None;
        }

        log::info!(
            "Server is running on {}:{}",
            config.network_interface,
            config.network_port
        );

        Some(Server {
            encoder: BincodeEncoder::new(),
            objects_rwlock,
            inputs_rwlock: input_rwlock,
            listener: Some(listener),
            subscriptions: HashSet::new(),
            node,
        })
    }

    fn send_to_all_clients(&mut self, endpoints: Vec<Endpoint>, message: FromServerMessage) {
        let data = self.encoder.encode(message);
        for endpoint in endpoints {
            self.node.network().send(endpoint, data);
        }
    }

    pub fn run(mut self) {
        let listener = self.listener.take().unwrap();
        self.node.signals().send(Signal::Update);
        listener.for_each(move |event| match event {
            NodeEvent::Signal(signal) => match signal {
                Signal::Close => {
                    log::info!("Closing server");
                    self.node.stop();
                }
                Signal::Update => {
                    let objects_guard = self.objects_rwlock.read().unwrap();
                    let inputs_guard = self.inputs_rwlock.read().unwrap();
                    let inputs = inputs_guard.clone();
                    let (ships, planets) = objects_guard.clone();
                    drop(objects_guard);
                    drop(inputs_guard);

                    let sent_state = State {
                        inputs,
                        planets,
                        ships,
                    };

                    let message = FromServerMessage::Update(sent_state);
                    let subscriptions = self.subscriptions.iter().cloned().collect();
                    self.send_to_all_clients(subscriptions, message);
                    self.node
                        .signals()
                        .send_with_timer(Signal::Update, *UPDATE_DURATION);
                }
            },
            NodeEvent::Network(network) => match network {
                NetEvent::Accepted(endpoint, _) => {
                    log::info!("Client connected: {}", endpoint);
                    self.subscriptions.insert(endpoint);
                }
                NetEvent::Connected(_, _) => (),
                NetEvent::Disconnected(endpoint) => {
                    log::info!("Client disconnected: {}", endpoint);
                    self.subscriptions.remove(&endpoint);
                }
                NetEvent::Message(endpoint, data) => {
                    match BincodeEncoder::decode::<FromClientMessage>(data) {
                        Some(message) => {
                            let mut guard = self.inputs_rwlock.write().unwrap();
                            let mut inputs = guard.deref_mut();
                            match message {
                                FromClientMessage::NavMode(navmode) => {
                                    inputs.navmode = navmode;
                                }
                                FromClientMessage::Throttle(throttle) => {
                                    inputs.throttle = throttle;
                                }
                            }
                        }
                        None => {
                            log::error!(
                                "Failed to decode message: {} sent an unknown messsage",
                                endpoint
                            );
                        }
                    }
                }
            },
        });
    }
}
