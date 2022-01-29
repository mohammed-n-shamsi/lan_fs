use super::config;
use super::util;
use std::net::{TcpListener, TcpStream};

struct HeaderInfo {
    command: util::Commands,
    size: u64,
}

pub struct ClientInfo {
    stream: TcpStream,
    client_header: HeaderInfo,
}

// Error constants
const BIND_ERROR: &str = "ERR: Failed to bind";
const STREAM_ERROR: &str = "ERR: Stream Failure";

// Global constants
const DEFAULT_HEADER: HeaderInfo = HeaderInfo {
    command: util::Commands::Unknown,
    size: 0,
};

/* Header structure:
 *      <Command> <Size> \r\n\r\n
 *      <Command>  => One Byte: &str
 *      < >        => One Byte: &str
 *      <Size>     => Eight Bytes: Unsigned Integer
 *      < >        => One Byte: &str
 *      <\r\n\r\n> => Four Bytes: &str
 *      --------------------------------------------
 *      Total Size => Fifteen Bytes
 *
 */

pub fn server_read_header(client: ClientInfo) -> util::Commands {
    return client.client_header.command;
}

pub fn server_create(handler_callback: &dyn Fn(ClientInfo)) {
    let listener = TcpListener::bind(config::SERVER_PORT).expect(BIND_ERROR);

    for stream in listener.incoming() {
        let client: ClientInfo = ClientInfo {
            stream: stream.expect(STREAM_ERROR),
            client_header: DEFAULT_HEADER,
        };
        handler_callback(client);
    }
}
