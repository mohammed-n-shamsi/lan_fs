// Only use network utility bindings
// to abstract network backend
use crate::utility::net_util;
use crate::utility::util;

pub fn serve() {
    net_util::server_create(&client_handler);
}

fn client_handler(client: net_util::ClientInfo) {
    let client_request: util::Commands = net_util::server_read_header(client);
}
