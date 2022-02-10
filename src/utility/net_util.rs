use super::config;
use super::util;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

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
const READ_ERROR: &str = "ERR: Failed to read";
const SIZE_TL_ERROR: &str = "ERR: Failed to translate size from bytes";
// Global constants
const DEFAULT_HEADER: HeaderInfo = HeaderInfo {
    command: util::Commands::Unknown,
    size: 0,
};

const HEADER_LENGTH: usize = 15;
const COMMAND_IDX: usize = 0;
const SIZE_LOWER_INC_IDX: usize = 2;
const SIZE_UPPER_EXC_IDX: usize = 10;

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

pub fn server_read_header(mut client: ClientInfo) -> util::Commands {
    let mut header_msg_buf = [0; HEADER_LENGTH];
    client.stream.read(&mut header_msg_buf).expect(READ_ERROR);

    // Assume clean data for now
    // first byte is command, next is space, then comes first int size
    // it will be [2 .. 10] "00000000" because last position is one more
    let unparsed_size_bytes: &[u8] = &header_msg_buf[SIZE_LOWER_INC_IDX..SIZE_UPPER_EXC_IDX];
    let unparsed_size_str: &str = from_utf8(unparsed_size_bytes).expect(SIZE_TL_ERROR);
    client.client_header.size = unparsed_size_str.parse::<u64>().expect(SIZE_TL_ERROR);

    // Parse command
    let unparsed_command: char = header_msg_buf[COMMAND_IDX] as char;
    const CALLER: util::Caller = util::Caller::Client;

    // Match command enums to parsed command char off the network
    client.client_header.command = match unparsed_command {
        // Handle the network based client calls
        'r' => util::Commands::ClientCall(util::ClientCommands::Replicate),
        'i' => util::Commands::ClientCall(util::ClientCommands::Integrate),
        // Handle the network based common calls
        'l' => util::Commands::CommonCall(util::CommonCommands::List, CALLER),
        'h' => util::Commands::CommonCall(util::CommonCommands::History, CALLER),
        's' => util::Commands::CommonCall(util::CommonCommands::Stats, CALLER),
        _ => util::Commands::Unknown,
    };

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
