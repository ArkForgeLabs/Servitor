use actix_web::{
    get, post,
    web::{self, Json},
    HttpRequest, HttpResponse, Responder,
};
use serde::de::Error;

pub mod account;
pub mod nodes;

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

    HttpResponse::Ok()
}

#[post("/{service}/create")]
pub async fn create(service: web::Path<String>, data: String) -> actix_web::Result<HttpResponse> {
    // Print the raw JSON data for debugging purposes
    println!("raw {:?}", data);

    /// A function to check if a given JSON string can be deserialized into a specified type T.
    ///
    /// # Arguments
    ///
    /// * `json_input` - The input JSON string to parse
    ///
    /// # Returns
    ///
    /// * `Result<serde_json::Value, serde_json::Error>` - An Ok variant containing the parsed JSON value if it is a valid object and can be deserialized into type T. Otherwise, an Err variant with a custom error message.
    fn typecheck<T: for<'de> serde::Deserialize<'de>>(
        json_input: &str,
    ) -> Result<serde_json::Value, serde_json::Error> {
        // Parse the input JSON string into a serde_json::Value object
        let json_form: serde_json::Value = serde_json::from_str(json_input)?;

        // Attempt to deserialize the JSON value into type T. If this fails, return an error.
        serde_json::from_value::<T>(json_form.clone())?;

        // Check if the parsed JSON value is an object. If it is not, return an error.
        if json_form.as_object().is_some() {
            Ok(json_form)
        } else {
            Err(serde_json::Error::custom("Invalid JSON"))
        }
    }

    // Match the service name and call the typecheck function with the appropriate struct type as a generic argument. If the service name is invalid, return an error.
    let result: Result<serde_json::Value, serde_json::Error> = match service.as_str() {
        "account" => typecheck::<crate::routesv1::account::Account>(&data),
        _ => Err(serde_json::Error::custom("Invalid JSON")),
    };

    // If the typecheck function returned an error, return a 404 Not Found response. Otherwise, return a successful HTTP response.
    if result.is_err() {
        return Err(actix_web::error::ErrorNotFound("Service not found"));
    }

    Ok(HttpResponse::Ok().finish())
}
