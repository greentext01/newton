use std::sync::Arc;

use crate::graphics::app::App;
use glutin_window::GlutinWindow;
use networking::{
    proto::net_state_client::NetStateClient, statekeeping::state::State, StateClient,
};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventSettings, Events, RenderEvent, UpdateEvent, WindowSettings};
use tokio::sync::RwLock;

mod graphics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state_container: Arc<RwLock<Option<State>>> = Arc::new(RwLock::new(None));
    let rpc_client = NetStateClient::connect("http://[::1]:50051").await?;
    let mut client = StateClient::new(rpc_client, state_container.clone());
    let opengl = OpenGL::V3_2;
    
    let mut window: GlutinWindow = WindowSettings::new("Newton", [800, 500])
    .graphics_api(opengl)
    .build()
    .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        state: None,
    };

    tokio::spawn(async move {
        loop {
            client.start_stream().await.unwrap_or_else(|e| {
                println!("Error: {:?}\n Restarting...", e);
            });
        }
    });

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(_args) = e.update_args() {
            let state_read = state_container.read().await;
            app.state = state_read.clone();
        }
    }

    Ok(())
}
