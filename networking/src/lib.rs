pub mod proto {
    tonic::include_proto!("newton");
}

pub mod statekeeping;
use std::borrow::BorrowMut;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::Arc;

use proto::net_state_client::NetStateClient;
use proto::net_state_server::{NetState, NetStateServer};
use proto::{Command, State as ProtoState};
use statekeeping::state::State;
use tokio::runtime::Runtime;
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
        request: Request<()>,
    ) -> Result<Response<Self::GetStateStream>, Status> {
        let (mut tx, rx) = mpsc::channel(4);
        let state = self.state.clone();

        tokio::spawn(async move {
            while state.lock().await.changed().await.is_ok() {
                let state_mutex = state.lock().await;
                let state = state_mutex.borrow().clone();
                tx.send(Ok(ProtoState::from(state))).await.unwrap();
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

    /// A channel to recieve commands from the server.
    /// This should be created alongside with a sender, to be used in the program.
    commands_rx: watch::Receiver<Command>,

    /// The current state of the simulation. Saved by the `start_stream` function.
    /// Behind a RwLock to avoid race conditions.
    /// Behind an Arc to allow async streams to access it.
    state_lock: Arc<RwLock<State>>,
}

impl StateClient {
    async fn start_stream(&mut self) -> Result<(), Box<dyn Error>> {
        let state_container = self.state_lock.clone();
        let mut stream = self.client.get_state(Request::new(())).await?.into_inner();

        while let Some(feature) = stream.message().await? {
            let mut state_ref = state_container.write().await;
            let state = feature.try_into();
            if let Ok(state) = state {
                *state_ref = state;
            }
        }

        Ok(())
    }

    fn get_state(&mut self) -> State {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(self.state_lock.read()).clone()
    }
}
