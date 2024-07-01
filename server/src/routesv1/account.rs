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
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub referral: Option<String>,
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
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[post("/create_account")]
pub async fn create_account(
    app_data: web::Data<crate::AppState>,
    account: web::Json<AccountUpdate>,
) -> actix_web::Result<HttpResponse> {
    let account = account.into_inner();
    let password = crate::utils::hash(&account.password.as_str());

    match sqlx::query("INSERT INTO users (username, email, password, creation_date) VALUES ($1, $2, $3, CURRENT_TIMESTAMP)")
        .bind(account.username.clone())
        .bind(account.email.clone())
        .bind(password)
        .execute(&app_data.database.pool)
        .await
    {
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
    query: actix_web::web::Query<AccountLogin>,
    app_data: web::Data<crate::AppState>,
    token_signer: web::Data<
        actix_jwt_auth_middleware::TokenSigner<VerifiedAccount, jwt_compact::alg::Ed25519>,
    >,
) -> actix_jwt_auth_middleware::AuthResult<HttpResponse> {
    let password = crate::utils::hash(query.password.as_str());

    let user =
        sqlx::query_as::<_, Account>("SELECT * FROM users WHERE email = $1 AND password = $2;")
            .bind(query.email.clone())
            .bind(password)
            .fetch_one(&app_data.database.pool)
            .await;

    if user.is_err() {
        println!("Error fetching user account: {:?}", user.as_ref().err());

        return Err(actix_jwt_auth_middleware::AuthError::TokenValidation(
            jwt_compact::ValidationError::InvalidSignature,
        ));
    }

    let user = user.unwrap();
    let user = VerifiedAccount {
        id: user.id,
        username: user.username,
        email: user.email,
    };

    Ok(HttpResponse::Ok()
        .cookie(token_signer.create_access_cookie(&user)?)
        .cookie(token_signer.create_refresh_cookie(&user)?)
        .body("You are now logged in"))
}
