pub enum Caller {
    Server,
    Client,
}

pub enum CommonCommands {
    List,
    History,
    Stats,
}

pub enum ServerCommands {
    Mount,
    Unmount,
}

pub enum ClientCommands {
    Replicate,
    Integrate,
    Clean,
}

pub enum Commands {
    CommonCall(CommonCommands, Caller),
    ServerCall(ServerCommands),
    ClientCall(ClientCommands),
    Unknown,
}

pub fn determine_command(arguments: Vec<String>) -> Commands {
    // Length Definitions
    const COMMON_ARG_LENGTH: usize = 3;
    const DEFINED_ARG_LENGTH: usize = 2;

    // Index Access Definitions
    const COMMON_CALLER_INDEX: usize = 1;
    const COMMON_COMMAND_INDEX: usize = 2;
    const DEFINED_COMMAND_INDEX: usize = 1;

    // Argument Definitions
    const COMMON_CALLER_SERVER: &str = "s";
    const COMMON_CALLER_CLIENT: &str = "c";

    const COMMON_COMMAND_LIST: &str = "l";
    const COMMON_COMMAND_HISTORY: &str = "h";
    const COMMON_COMMAND_STATS: &str = "s";

    const DEFINED_SERVER_MOUNT: &str = "m";
    const DEFINED_SERVER_UNMOUNT: &str = "u";

    const DEFINED_CLIENT_REPLICATE: &str = "r";
    const DEFINED_CLIENT_INTEGRATE: &str = "i";
    const DEFINED_CLIENT_CLEAN: &str = "c";

    // Begin processing

    // Default command to Unknown
    let mut command: Commands = Commands::Unknown;

    if arguments.len() == COMMON_ARG_LENGTH {
        let is_server: bool = arguments[COMMON_CALLER_INDEX].eq(COMMON_CALLER_SERVER);
        let is_client: bool = arguments[COMMON_CALLER_INDEX].eq(COMMON_CALLER_CLIENT);

        if is_client || is_server {
            let is_list: bool = arguments[COMMON_COMMAND_INDEX].eq(COMMON_COMMAND_LIST);
            let is_history: bool = arguments[COMMON_COMMAND_INDEX].eq(COMMON_COMMAND_HISTORY);
            let is_stats: bool = arguments[COMMON_COMMAND_INDEX].eq(COMMON_COMMAND_STATS);

            if is_client {
                if is_list {
                    command = Commands::CommonCall(CommonCommands::List, Caller::Client);
                } else if is_history {
                    command = Commands::CommonCall(CommonCommands::History, Caller::Client);
                } else if is_stats {
                    command = Commands::CommonCall(CommonCommands::Stats, Caller::Client);
                }
            } else {
                if is_list {
                    command = Commands::CommonCall(CommonCommands::List, Caller::Server);
                } else if is_history {
                    command = Commands::CommonCall(CommonCommands::History, Caller::Server);
                } else if is_stats {
                    command = Commands::CommonCall(CommonCommands::Stats, Caller::Server);
                }
            }
        }
    } else if arguments.len() == DEFINED_ARG_LENGTH {
        let is_mount: bool = arguments[DEFINED_COMMAND_INDEX].eq(DEFINED_SERVER_MOUNT);
        let is_unmount: bool = arguments[DEFINED_COMMAND_INDEX].eq(DEFINED_SERVER_UNMOUNT);
        let is_replicate: bool = arguments[DEFINED_COMMAND_INDEX].eq(DEFINED_CLIENT_REPLICATE);
        let is_integrate: bool = arguments[DEFINED_COMMAND_INDEX].eq(DEFINED_CLIENT_INTEGRATE);
        let is_clean: bool = arguments[DEFINED_COMMAND_INDEX].eq(DEFINED_CLIENT_CLEAN);

        if is_mount {
            command = Commands::ServerCall(ServerCommands::Mount);
        } else if is_unmount {
            command = Commands::ServerCall(ServerCommands::Unmount);
        } else if is_replicate {
            command = Commands::ClientCall(ClientCommands::Replicate);
        } else if is_integrate {
            command = Commands::ClientCall(ClientCommands::Integrate);
        } else if is_clean {
            command = Commands::ClientCall(ClientCommands::Clean);
        }
    }

    return command;
}
