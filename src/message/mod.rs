use std::raw::Slice;
use std::mem;
use std::fmt::String;
use std::io;
use std::io::net::pipe::UnixStream;
use std::io::IoResult;

pub trait Message {
    fn print_mesg(&self);
    fn get_stream(&self) -> IoResult<UnixStream>;
} 

fn mesg_bytes<'a, T: Message>(mesg: T) -> &'a [u8] {
    let const_ptr: *const T = &mesg;
    let mesg_as_bytes: *const u8 = const_ptr as *const u8;

    let raw_bytes = std::raw::Slice {
        data: mesg_as_bytes,
        len: std::mem::size_of::<T>()
    };

    //let bytes_mesg: &[u8] = unsafe {
        //std::mem::transmute(std::raw::Slice {
    //            };
    //return bytes_mesg;

}
pub fn send_message<M: Message>(mesg: M) ->IoResult<()> {

    let m_bytes = mesg_bytes(mesg);
    let stream = mesg.get_stream().unwrap();
    return stream.write(m_bytes); 
}
