use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use serde_json::to_value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Account {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/create_account")]
pub async fn create_account(
    app_data: web::Data<crate::AppState>,
    account: web::Json<Account>,
) -> actix_web::Result<HttpResponse> {
    let account = account.into_inner();
    let query = "INSERT INTO users (username, email, password) VALUES (?, ?, ?)";
    let params = vec![
        to_value(account.username.clone())?,
        to_value(account.email.clone())?,
        to_value(account.password.clone())?,
    ];

    match app_data.database.query(query.to_string(), params) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Failed to create account",
        )),
    }
}
