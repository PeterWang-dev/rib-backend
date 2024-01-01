#[test]
fn test_rib_server() {
    use rib_server::message::Message;

    let msg = Message::new("Alice".to_string(), "Hello world!".to_string());
    println!("{}", msg);
    assert_eq!(msg.name(), "Alice");
    assert_eq!(msg.content(), "Hello world!");
}
