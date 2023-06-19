use std::sync::Arc;

use networking::{proto::{net_state_client::NetStateClient}, StateClient, statekeeping::state::State};
use tokio::sync::RwLock;

mod graphics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state_container: Arc<RwLock<Option<State>>> = Arc::new(RwLock::new(None));
    let rpc_client = NetStateClient::connect("http://[::1]:50051").await?;
    let mut client = StateClient::new(rpc_client, state_container.clone());
    

    tokio::spawn(async move {
        loop {
            client
                .start_stream()
                .await
                .unwrap_or_else(|e| {
                    println!("Error: {:?}\n Restarting...", e);
                });
        }
    });

    loop {
        let state_read = state_container.read().await;
        if let Some(state) = state_read.as_ref() {
            println!("State: {:?}", state);
        }
    }
}
