use std::io::net::ip::{ToSocketAddr};
use std::io::{IoResult, TcpStream} ;
pub use self::host::Host;
pub use self::client::Client;
pub mod host;
pub mod client;

pub struct TestStruct { attr: bool }

trait UniSocket {
    fn connect(ToSocketAddr) -> IoResult<TcpStream>;
}


