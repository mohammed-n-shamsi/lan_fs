use std::env;

mod client;
mod common;
mod server;
mod utility;

use crate::client::client_call;
use crate::common::common_call;
use crate::server::server_call;
use crate::utility::util;

fn main() {
    // Argument definition string
    let help_str: &str = "Argument structure
    Common calls:
     ./<app_name> <caller> <command>
     <caller>:
              's':Server,
              'c':Client
     <command>:
               'l':List,
               'h':History,
               's':Stats,
    
    Server calls:
     ./<app_name> <command>
     <command>:
               'm':Mount,
               'u':Unmount
    
    Client calls:
     ./<app_name> <command>
     <command>:
               'r':Replicate,
               'i':Integrate,
               'c':Clean
    \n";

    // Take in args
    let args: Vec<String> = env::args().collect();
    let command: util::Commands = util::determine_command(args);

    // Match commands to respective sub-app and call the exposed function
    // to begin execution of command, if valid
    match command {
        util::Commands::CommonCall(cmd, caller_type) => common_call::call(cmd, caller_type),
        util::Commands::ServerCall(cmd) => server_call::call(cmd),
        util::Commands::ClientCall(cmd) => client_call::call(cmd),
        util::Commands::Unknown => println!("{:}", help_str),
    }
}
