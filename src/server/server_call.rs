use super::server_lib;
use crate::utility::fs_util;
use crate::utility::util;

pub fn call(command: util::ServerCommands) {
    match command {
        util::ServerCommands::Mount => {
            fs_util::mount_dir();
            server_lib::initialize_server();
            server_lib::serve();
        }
        util::ServerCommands::Unmount => {
            fs_util::unmount_dir();
        }
    }
}
