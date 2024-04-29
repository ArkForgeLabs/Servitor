use actix_web::{
    get, post,
    web::{self},
    HttpRequest, HttpResponse,
};
use serde_json::to_value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Account {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VerifiedAccount {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[post("/create_account")]
pub async fn create_account(
    app_data: web::Data<crate::AppState>,
    account: web::Json<Account>,
) -> actix_web::Result<HttpResponse> {
    let account = account.into_inner();
    let query = crate::utils::CREATE_USER;
    let params = vec![
        to_value(account.username.clone())?,
        to_value(account.email.clone())?,
        to_value(account.password.clone())?,
    ];

    match app_data.database.query(query, params) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Failed to create account",
        )),
    }
}

#[get("/get_account")]
pub async fn get_account(
    app_data: web::Data<crate::AppState>,
    request: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let verified = crate::utils::verify(&app_data.database, request);
    if verified.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }

    Ok(HttpResponse::Ok().finish())
}
