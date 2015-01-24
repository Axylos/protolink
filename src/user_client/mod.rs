pub fn hello() -> &'static str{
    return "hello";
}

#[test]
fn test_hello() {
    assert_eq!(hello(), "hello");
}
