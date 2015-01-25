use std::io::net::ip::{ToSocketAddr};
use std::io::{IoResult, TcpStream} ;
pub use self::host::Host;
pub use self::client::Client;
pub use self::uni_mesg::UniMesg;
pub mod host;
pub mod client;
pub mod uni_mesg;

trait UniSocket {
    fn connect(ToSocketAddr) -> IoResult<TcpStream>;
}


