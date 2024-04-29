const DATABASE_INIT: &str = r#"
CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, username TEXT, email TEXT, password TEXT);
CREATE TABLE IF NOT EXISTS graphs (id INTEGER PRIMARY KEY, graph_name TEXT, user INTEGER NOT NULL, nodes: TEXT);
"#;

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

#[derive(Debug)]
pub struct Database {
    pub sender: std::sync::mpsc::Sender<Message>,
}
impl Database {
    pub fn new(
        path: String,
        sender: std::sync::mpsc::Sender<Message>,
        receiver: std::sync::mpsc::Receiver<Message>,
    ) -> Self {
        std::thread::spawn(move || {
            let connection = rusqlite::Connection::open(path).expect("Failed to open the database");
            connection
                .execute(DATABASE_INIT, [])
                .expect("Failed to initialize database");

            loop {
                let message = receiver.recv();

                if message.is_ok() {
                    let message = message.unwrap();

                    match message {
                        Message::Query(query, params, sender) => {
                            let result = connection
                                .execute(query.as_str(), rusqlite::params_from_iter(params));

                            let result = match result {
                                Ok(value) => {
                                    Ok(serde_json::to_value(value).expect("Failed to serialize"))
                                }
                                Err(err) => Err(err),
                            };

                            sender
                                .send(Message::Response(result))
                                .expect("Failed to send response");
                        }
                        Message::Ping(id, message, sender) => {
                            sender
                                .send(Message::Pong(id, message))
                                .expect("Failed to send response");
                        }
                        _ => {}
                    };
                }
            }
        });

        Self { sender }
    }

    pub fn query(
        &self,
        query: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, rusqlite::Error> {
        let (sender, receiver) = std::sync::mpsc::channel();

        if self
            .sender
            .send(Message::Query(query.to_string(), params, sender))
            .is_err()
        {
            return Err(rusqlite::Error::InvalidQuery);
        }

        match receiver.recv() {
            Ok(Message::Response(result)) => result,
            _ => Err(rusqlite::Error::InvalidQuery),
        }
    }
}
