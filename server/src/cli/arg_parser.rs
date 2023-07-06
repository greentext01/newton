use clap::Parser;

/// The server for Newton. It is responsible for running the simulation and sending the data to the clients.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// The file to load the initial state from.
    #[clap(short, long)]
    pub system: String,

    /// The network interface to bind to.
    #[clap(short, long, default_value = "0.0.0.0")]
    pub interface: String,

    /// The network port to bind to.
    #[clap(short, long, default_value = "5000")]
    pub port: u16,
    
    /// The amount of softening applied to the simulation.
    #[clap(long, default_value = "0.1")]
    pub softening: f64,
    
    /// The number of updates per second to run the simulation at.
    #[clap(short, long, default_value = "60")]
    pub updates_per_second: u32,
    
    /// The target updates per second for the physics engine.
    #[clap(short, long, default_value = "60")]
    pub target_fps: u32,

    /// The minimum steps per update for the physics engine.
    #[clap(long, default_value = "10")]
    pub min_spu: u32,

    /// The maximum steps per update for the physics engine.
    #[clap(long)]
    pub max_spu: Option<u32>,

}
