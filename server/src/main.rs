use actix_web::{web, App, HttpServer};
use std::thread;

// components
mod database;
mod routesv1;
mod services;

#[derive(Debug)]
pub struct AppState {
    pub services: std::sync::Mutex<services::Services>,
    pub sender: std::sync::mpsc::Sender<database::Message>,
}
/*
#[get("/test")]
pub async fn test(sender: Data<AppState>) -> actix_web::Result<HttpResponse> {
    sender
        .sender
        .send(Message::Hello("test".to_string()))
        .unwrap();

    Ok(HttpResponse::Ok().finish())
}
*/

#[actix_web::get("/test")]
pub async fn test(app: web::Data<AppState>) -> actix_web::Result<actix_web::HttpResponse> {
    let (send, receive) = std::sync::mpsc::channel::<database::Message>();

    app.sender
        .send(database::Message::Ping(
            "test".to_string(),
            "hello".to_string(),
            send,
        ))
        .unwrap();

    let response = receive.recv();
    println!("Response: {:?}", response);

    Ok(actix_web::HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (send, receiver) = std::sync::mpsc::channel::<database::Message>();

    let app_data = web::Data::new(AppState {
        services: std::sync::Mutex::new(services::Services::new()),
        sender: send.clone(),
    });

    database::database("database.db".to_string(), receiver);

    let heartbeat_clone = app_data.clone();
    thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let mut services = heartbeat_clone.services.lock().unwrap();
        services.heartbeat();
    });

    HttpServer::new(move || {
        App::new()
            .service(routesv1::index)
            .service(
                web::scope("/apiv1")
                    .service(routesv1::get_service)
                    .service(routesv1::create_service)
                    .service(test),
            )
            .app_data(app_data.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
