use actix_web::{web, App, HttpServer};

// components
mod routesv1;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/apiv1").service(routesv1::index))
            .app_data(web::Data::new(Vec::<services::Service>::new()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
