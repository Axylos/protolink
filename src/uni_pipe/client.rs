use std::io::net::pipe::UnixStream;
use std::path::BytesContainer;
use std::io::IoResult;
use message::{Message, mesg_bytes};

#[derive(Clone)]
pub struct Client { stream: UnixStream }

pub struct UniMesg { mesg: String }

impl UniMesg {
    pub fn new(text: String) -> UniMesg {
        UniMesg { mesg: text }
    }
}

impl Message for UniMesg {
}

impl Client {
    fn new<P: BytesContainer>(server: P) -> IoResult<Client> {
        let mut stream = UnixStream::connect(&server);
        match  stream {
            Ok(ok) => {
                return Ok(Client { stream: ok });
            }
            Err(e) => {
                println!("could not connect to address");
                return Err(e);
            }
        }
    }

    fn send_message(&self, mesg: UniMesg) -> IoResult<()> {
        let mut stream = self.stream.clone();
        let m_bytes = mesg_bytes(mesg);
        let result = stream.write(m_bytes); 
        return result;
    }
}
