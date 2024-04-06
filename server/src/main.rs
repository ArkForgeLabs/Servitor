use actix_web::{web, App, HttpServer};

// components
mod routesv1;
mod services;

pub struct AppState {
    pub services: std::sync::Mutex<services::Services>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        services: std::sync::Mutex::new(services::Services::new()),
    });

    let heartbeat_clone = app_data.clone();
    std::thread::spawn(move || loop {
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
