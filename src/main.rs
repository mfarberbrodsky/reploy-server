fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let receive_messages = message_protocol::listen_to_tcp("127.0.0.1:37255")?;
    while let Ok(msg) = receive_messages.recv() {
        println!("from {}", msg.address);
        match msg.message {
            message_protocol::Message::Open => println!("open"),
            message_protocol::Message::Bytes(b) => println!("bytes {:?}", b),
            message_protocol::Message::Close => println!("close"),
        };
    }
    Ok(())
}
