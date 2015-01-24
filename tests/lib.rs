extern crate protolink;
use protolink::uni_pipe;
use protolink::uni_pipe::{ TestStruct, Client, Host};
use protolink::uni_pipe::client::UniMesg;

#[test]
fn it_works() {
    let ans = 2 + 2;
    assert_eq!(4, ans);
}

#[test]
fn uni_pipe_functions_are_there() {
    assert_eq!(uni_pipe::host::collect_messages(), "it works");
}



#[cfg(test)]
fn can_send_and_receive() {
    //define config attrs
    let msg = UniMesg::new("some text".to_string()); 
    let path = "/var/run/rust";

    //initialize and connect server and client
    let host_messenger = Host::new(path);
    let client_messenger = Client::new(path).unwrap();
    client_messenger.send_message(msg);

    //check if message was send and received ok
    let received_msg =  host_messenger.get_one_message();
    assert_eq!(msg, received_msg);
}
