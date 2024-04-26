#[derive(Debug)]
pub enum Message {
    Query(String, Vec<serde_json::Value>),
    Response(Result<serde_json::Value, rusqlite::Error>),
}

pub fn database(
    path: String,
    sender: crossbeam_channel::Sender<Message>,
    receiver: crossbeam_channel::Receiver<Message>,
) {
    std::thread::spawn(move || {
        let connection = rusqlite::Connection::open(path).unwrap();
        loop {
            let message = receiver.recv().unwrap();
            match message {
                Message::Query(query, params) => {
                    let result =
                        connection.execute(query.as_str(), rusqlite::params_from_iter(params));

                    let result = match result {
                        Ok(value) => Ok(serde_json::to_value(value).unwrap()),
                        Err(err) => Err(err),
                    };

                    sender.send(Message::Response(result)).unwrap();
                }
                _ => {}
            };
        }
    });
}
