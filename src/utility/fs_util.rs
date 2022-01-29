use super::config;
use std::fs;

pub fn mount_dir() {
    // Create fs dir if doesn't exist
    fs::create_dir_all(config::MOUNT_DIR).expect("ERR: Unable to Mount");
}

pub fn unmount_dir() {
    // Unmounts and removes the directory
    fs::remove_dir_all(config::MOUNT_DIR).expect("ERR: Unable to Unmount")
}
