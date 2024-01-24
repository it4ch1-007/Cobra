use clap::Parser;
use super::api;

mod server;
use server::SubCommands;


pub fn parse_cmdline_args() {
    let server_args = server::ServerArgs::parse();
    match server_args.cmd {
        SubCommands::RunApiServer => {
            api::start_api_server()
        }
        SubCommands::ListAllConnections => {}
        SubCommands::ListOnlineConnections => {}
    }
}
