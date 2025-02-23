use clap::{Parser,Subcommand};

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct ServerArgs{
    #[command(subcommand)]
    pub cmd:SubCommands
}

#[derive(Subcommand,Debug,Clone)]
pub enum SubCommands{
    #[command(about = "Run the API server")]
    RunApiServer,
    #[command(about = "List all targets")]
    ListAllConnections,
    #[command(about = "List all online targets")]
    ListOnlineConnections
}