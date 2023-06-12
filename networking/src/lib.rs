pub mod proto {
    tonic::include_proto!("newton");
}

pub mod statekeeping;
use std::collections::VecDeque;
use std::pin::Pin;
use std::{sync::Arc};

use crate::proto::{state_server_server::StateServer, Command, State as NetState};
use futures_core::Stream;
use proto::command::CommandIdent;
use proto::state_server_client::StateServerClient;
use statekeeping::state::State;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct MyStateServer {
    /// The current state of the simulation
    state: Arc<statekeeping::state::State>,

    /// A Sender to send commands to the simulation
    command_queue_tx: mpsc::Sender<Command>,
}

#[tonic::async_trait]
impl StateServer for MyStateServer {
    type GetStateStream = Pin<Box<dyn Stream<Item = Result<NetState, Status>> + Send + 'static>>;

    async fn get_state(
        &self,
        request: Request<tonic::Streaming<Command>>,
    ) -> Result<Response<Self::GetStateStream>, Status> {
        let mut stream = request.into_inner();
        let state = self.state.clone();
        let command_queue_tx = self.command_queue_tx.clone();

        let output = async_stream::try_stream! {
            while let Some(command) = stream.next().await {
                let command = command?;

                if command.ident != CommandIdent::Noop as i32 {
                    command_queue_tx.send(command).await.unwrap();
                }

                yield state.as_ref().clone().into();
            }
        };

        Ok(Response::new(Box::pin(output) as Self::GetStateStream))
    }
}

pub struct StateServerClientWrapper {
    client: StateServerClient<Channel>,
    commands: VecDeque<Command>,
}

impl StateServerClientWrapper {
    fn get_state(&self) -> State {
        self.client.get_state(request);
    }
}
