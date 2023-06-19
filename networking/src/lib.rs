pub mod proto {
    tonic::include_proto!("newton");
}

pub mod statekeeping;
use std::error::Error;
use std::sync::Arc;

use proto::net_state_client::NetStateClient;
use proto::net_state_server::NetState;
use proto::{Command, State as ProtoState};
use statekeeping::state::State;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::{mpsc, watch, Mutex, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct MyStateServer {
    /// The current state of the simulation
    pub state: Arc<Mutex<watch::Receiver<State>>>,

    /// A queue where commands will be sent
    pub command_queue_tx: mpsc::Sender<Command>,
}

#[tonic::async_trait]
impl NetState for MyStateServer {
    type GetStateStream = ReceiverStream<Result<ProtoState, Status>>;

    async fn get_state(
        &self,
        _request: Request<()>,
    ) -> Result<Response<Self::GetStateStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let state = self.state.clone();

        tokio::spawn(async move {
            'state_loop: while state.lock().await.changed().await.is_ok() {
                let state_mutex = state.lock().await;
                let state = state_mutex.borrow().clone();
                match tx.send(Ok(ProtoState::from(state))).await {
                    Err(_) => {
                        break 'state_loop;
                    }
                    Ok(_) => {}
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

/// A client for the state server.
/// Sends commands to the server, and records state in a variable.
///
/// The client can access the variable at any time.
pub struct StateClient {
    /// The RPC client.
    client: NetStateClient<Channel>,

    /// The current state of the simulation. Saved by the `start_stream` function.
    /// Behind a RwLock to avoid race conditions.
    /// Behind an Arc to allow async streams to access it.
    pub state_container: Arc<RwLock<Option<State>>>,
}

impl StateClient {
    pub fn new(
        client: NetStateClient<Channel>,
        state_container: Arc<RwLock<Option<State>>>,
    ) -> StateClient {
        StateClient {
            client,
            state_container,
        }
    }

    pub async fn start_stream(&mut self) -> Result<(), Box<dyn Error>> {
        let state_container = Arc::clone(&self.state_container);
        let mut stream = self.client.get_state(Request::new(())).await?.into_inner();

        while let Some(feature) = stream.message().await? {
            // Gets the state with write access. state_ref is a locked reference to the state.
            let mut state_ref = state_container.write().await;

            // Converts NetState to State
            let state: Result<State, _> = feature.try_into();
            if let Ok(state) = state {
                *state_ref = Some(state);
            }
        }

        Ok(())
    }

    pub async fn get_state(&mut self) -> Option<State> {
        self.state_container.read().await.clone()
    }
}
