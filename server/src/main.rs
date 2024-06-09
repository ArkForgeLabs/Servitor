use actix_web::{middleware::Logger, web, App, HttpServer};
use std::thread;

// components
mod database;
mod routesv1;
mod services;
mod utils;

#[derive(Debug)]
pub struct AppState {
    pub services: std::sync::Mutex<services::Services>,
    pub database: database::Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let database = database::Database::new()
        .await
        .expect("Failed to initialize database");

    let app_data = web::Data::new(AppState {
        services: std::sync::Mutex::new(services::Services::new().await),
        database,
    });

    let heartbeat_clone = app_data.clone();
    thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let mut services = heartbeat_clone.services.lock().unwrap();
        services.heartbeat();
    });

    let server = HttpServer::new(move || {
        App::new()
            .service(routesv1::index)
            .service(
                web::scope("/apiv1")
                    .service(routesv1::get_service)
                    .service(routesv1::create_service)
                    .service(routesv1::create)
                    .service(routesv1::get_all)
                    // account
                    .service(routesv1::account::create_account)
                    .service(routesv1::account::get_account),
            )
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(app_data.clone())
    });
    let server = server.bind(("0.0.0.0", 8080))?;

    println!("started server at: 0.0.0.0:{}", 8080);

    server.run().await
}
