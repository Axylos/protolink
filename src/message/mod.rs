use std::raw::Slice;
use std::mem;
use std::fmt::String;
use std::io;
use std::io::net::pipe::UnixStream;
use std::io::IoResult;

pub trait Message:Copy {
    fn print_mesg(&self);
    fn get_stream(&self) -> IoResult<UnixStream>;
} 

fn mesg_bytes<'a, T: Message>(mesg: T) -> &'a [u8] {
    let const_ptr: *const T = &mesg;
    let mesg_as_bytes: *const u8 = const_ptr as *const u8;

    let raw_bytes = Slice {
        data: mesg_as_bytes,
        len: mem::size_of::<T>()
    };

    let bytes_mesg: &[u8] = unsafe {
        mem::transmute(raw_bytes)
    };
    return bytes_mesg;

}
pub fn send_message<M: Message>(mesg: M) ->IoResult<()> {

    let m_bytes = mesg_bytes(mesg);
    let mut stream = mesg.get_stream().unwrap();
    let result = stream.write(m_bytes); 
    return result;
}
