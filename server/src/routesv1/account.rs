use actix_web::{
    delete, get, post, put,
    web::{self, Query},
    HttpRequest, HttpResponse,
};
use serde_json::to_value;

/// Account related structs and handlers
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    sqlx::FromRow,
    actix_jwt_auth_middleware::FromRequest,
)]
pub struct Account {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct AccountUpdate {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct AccountLogin {
    pub email: String,
    pub password: String,
}

/// VerifiedAccount for returning verified user details after login or signup
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
    // TODO: Add password hashing and salt generation for security purposes.
    let account = account.into_inner();
    let query = crate::utils::CREATE_USER;
    let params = vec![
        to_value(account.username.clone())?,
        to_value(account.email.clone())?,
        to_value(crate::utils::hash(account.password.clone().as_str()))?,
    ];

    match app_data.database.query(query, params) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Failed to create account",
        )),
    }
}

#[put("/update_account")]
pub async fn update_account(
    app_data: web::Data<crate::AppState>,
    account: web::Json<AccountUpdate>,
    request: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let account = account.into_inner();
    crate::utils::verify(&app_data.database, request)?;

    let query = crate::utils::UPDATE_USER;
    let params = vec![
        to_value(account.username.clone())?,
        to_value(account.email.clone())?,
        to_value(crate::utils::hash(account.password.clone().as_str()))?,
    ];

    match app_data.database.query(query, params) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Failed to update account",
        )),
    }
}

#[delete("/delete_account")]
pub async fn delete_account(
    app_data: web::Data<crate::AppState>,
    account_id: Query<u32>,
    request: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    crate::utils::verify(&app_data.database, request)?;
    let query = crate::utils::DELETE_USER;
    let params = vec![to_value(*account_id)?];

    match app_data.database.query(query, params) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Failed to delete account",
        )),
    }
}

#[get("/get_account")]
pub async fn get_account(
    _account: Account,
    app_data: web::Data<crate::AppState>,
    request: HttpRequest,
) -> actix_web::Result<web::Json<VerifiedAccount>> {
    Ok(web::Json(crate::utils::verify(
        &app_data.database,
        request,
    )?))
}

#[get("/login")]
async fn login(
    token_signer: web::Data<
        actix_jwt_auth_middleware::TokenSigner<AccountLogin, jwt_compact::alg::Ed25519>,
    >,
) -> actix_jwt_auth_middleware::AuthResult<HttpResponse> {
    let user = AccountLogin {
        email: "asda".to_string(),
        password: "asdasd".to_string(),
    };

    Ok(HttpResponse::Ok()
        .cookie(token_signer.create_access_cookie(&user)?)
        .cookie(token_signer.create_refresh_cookie(&user)?)
        .body("You are now logged in"))
}
