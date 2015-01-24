use std::io::net::pipe::UnixStream;
use std::path::BytesContainer;
use std::io::IoResult;
use message::Message;

pub  struct Client { stream: UnixStream }

impl Client {
    fn new<P: BytesContainer>(server: P) -> IoResult<Client> {
        match UnixStream::connect(&server) {
            Ok(ok) => {
                return Ok(Client { stream: ok });
            }
            Err(e) => {
                println!("could not connect to address");
                return Err(e);
            }
        }
    }

    fn send_message<M: Message>(&self, mesg: M) ->IoResult {
        byte_mesg = mesg.text.as_bytes();
        return self.stream.write(byte_mesg); 
    }
            

}