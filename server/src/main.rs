use actix_web::{web, App, HttpServer};
use std::thread;

// components
mod database;
mod routesv1;
mod services;

#[derive(Debug)]
pub struct AppState {
    pub services: std::sync::Mutex<services::Services>,
    pub database: database::Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (send, receiver) = std::sync::mpsc::channel::<database::Message>();
    let database = database::Database::new("database.db".to_string(), send.clone(), receiver);

    let app_data = web::Data::new(AppState {
        services: std::sync::Mutex::new(services::Services::new()),
        database,
    });

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
                    .service(routesv1::create_service),
            )
            .app_data(app_data.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
