use std::io::net::ip::{ ToSocketAddr, SocketAddr };

pub fn collect_messages() -> &'static str {
    let result = "it works";
    return result;
}

pub struct Host { addr: SocketAddr }

impl Host {
    fn new<'a>(addr: &'a str) -> Host {
        Host { addr: addr.to_socket_addr().unwrap() }
    }
}
