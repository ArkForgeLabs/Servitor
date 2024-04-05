use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello world!"
}

#[get("/get_service/{service_id}")]
pub async fn get_service(
    service_id: web::Path<u16>,
    app_data: web::Data<crate::AppState>,
) -> actix_web::Result<Json<crate::services::Service>> {
    let services = app_data.services.lock().unwrap();
    println!("services: {:?}", services);
    let service = services.get_service(service_id.into_inner());

    if service.is_some() {
        Ok(Json(service.unwrap()))
    } else {
        Err(actix_web::error::ErrorNotFound("Service not found"))
    }
}

#[post("/create_service")]
pub async fn create_service(
    service: Json<crate::services::Service>,
    app_data: web::Data<crate::AppState>,
) -> impl Responder {
    let mut services = app_data.services.lock().unwrap();
    services.add_service(service.into_inner());

    println!("Services: {:?}", services);

    HttpResponse::Ok()
}
