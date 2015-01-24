use std::raw::Slice;
use std::mem;
use std::fmt::String;
use std::io;
use std::io::net::pipe::UnixStream;
use std::io::IoResult;

pub trait Message {
} 

pub fn mesg_bytes<'a, T: Message>(mesg: T) -> &'a [u8] {
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
