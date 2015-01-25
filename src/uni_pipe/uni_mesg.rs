use message::Message;
use std::raw::Slice;
pub struct UniMesg { mesg: String, dest: u32 }

impl UniMesg {
    pub fn new(text: String) -> UniMesg {
        return UniMesg { mesg: text, dest: 0 }
    }

}

impl Message for UniMesg {
}
