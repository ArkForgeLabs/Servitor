use actix_web::{
    get,
    web::{self, Json},
    Responder,
};

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello world!"
}

#[get("/get_service/{service_id}")]
pub async fn get_service(
    service_id: web::Path<u16>,
    services: web::Data<crate::services::Services>,
) -> Json<crate::services::Service> {
    let service = services.get_service(*service_id);

    Json(service)
}
