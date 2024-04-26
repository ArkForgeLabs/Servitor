#[derive(Debug)]
pub enum Message {
    Query(
        String,
        Vec<serde_json::Value>,
        std::sync::mpsc::Sender<Message>,
    ),
    Response(Result<serde_json::Value, rusqlite::Error>),
    Ping(String, String, std::sync::mpsc::Sender<Message>),
    Pong(String, String),
}

pub fn database(path: String, receiver: std::sync::mpsc::Receiver<Message>) {
    std::thread::spawn(move || {
        let connection = rusqlite::Connection::open(path).unwrap();
        loop {
            let message = receiver.recv().unwrap();
            println!("Received message: {:?}", message);
            match message {
                Message::Query(query, params, sender) => {
                    let result =
                        connection.execute(query.as_str(), rusqlite::params_from_iter(params));

                    let result = match result {
                        Ok(value) => Ok(serde_json::to_value(value).unwrap()),
                        Err(err) => Err(err),
                    };

                    sender.send(Message::Response(result)).unwrap();
                }
                Message::Ping(id, message, sender) => {
                    sender.send(Message::Pong(id, message)).unwrap();
                }
                _ => {}
            };
        }
    });
}
