extern crate protolink;
use protolink::uni_pipe;
use protolink::uni_pipe::{ TestStruct, Client, Host};

#[test]
fn it_works() {
    let ans = 2 + 2;
    assert_eq!(4, ans);
}

#[test]
fn uni_pipe_functions_are_there() {
    assert_eq!(uni_pipe::host::collect_messages(), "it works");
}

fn uni_pipe_structs_are_there() {
    let ts = TestStruct { attr: true };
    match ts.attr {
        true => assert!(true),
        false => assert!(false)
    }
}

#[cfg(test)]
fn can_send_and_receive() {
    //define config attrs
    let msg = "a message";
    let path = "/var/run/rust";

    //initialize and connect server and client
    let host_messenger = Host::new(path);
    let client_messenger = Client::new(path).unwrap();
    client_messenger.send_message(msg);

    //check if message was send and received ok
    let received_msg =  host_messenger.get_one_message();
    assert_eq!(msg, received_msg);
}
