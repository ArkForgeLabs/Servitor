use actix_web::{web, App, HttpServer};

// components
mod routesv1;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routesv1::index)
            .service(web::scope("/apiv1"))
            .app_data(web::Data::new(services::Services::new()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
