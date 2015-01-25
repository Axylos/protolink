use std::io::net::pipe::{UnixListener, UnixStream};
use std::path::BytesContainer;
use std::io::{FileType, IoResult, Listener, 
    Acceptor, BufferedReader};
use message::{Message, mesg_bytes};
use uni_pipe::uni_mesg::UniMesg;
use std::path::posix::Path;


#[derive(Clone)]
pub struct Host { pipe_path: Path }

impl Host {
    fn new<P: BytesContainer>(pipe_path: P) -> Option<Host> {
        let path = Path::new(pipe_path);
        match ::std::io::fs::lstat(&path) {
            Ok(ok) if ok.kind == FileType::NamedPipe => {
                let result = Host { pipe_path: path };
                return Some(result);
            },
            _ => return None
        }
    }

    fn get_one_message(&self) -> UniMesg{
        //let mut stream = self.listener.clone();
        let listener = UnixListener::bind(self.pipe_path).unwrap();
        let mut acceptor = listener.listen().unwrap();
        for mut client in acceptor.incoming() {
            let reader = BufferedReader::new(client.unwrap());
            let text = reader.read_until(b'\x04').unwrap();
            let dest = reader.read_until(b'\x01').unwrap().container_as_bytes();
            let rendered_dest = ::std::num::from_u8(dest[0]).unwrap();

            return UniMesg { 
                mesg: text.container_as_str().unwrap().to_string(), 
                dest: rendered_dest
            }
        }
    }
}

pub fn collect_messages() -> &'static str {
    let result = "it works";
    return result;
}
