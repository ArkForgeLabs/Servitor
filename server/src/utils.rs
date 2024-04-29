use actix_web::HttpRequest;
use sha2::Digest;

// SQL Queries
pub const CREATE_USER: &str = "INSERT INTO users (username, email, password) VALUES (?, ?, ?)";
pub const VERIFY_USER: &str = "SELECT id, username, email FROM users WHERE password = ?";

pub fn verify(
    database: &crate::database::Database,
    request: HttpRequest,
) -> Option<crate::routesv1::account::VerifiedAccount> {
    // get the auth header
    let auth_header = request.headers().get("Authorization");
    if auth_header.is_none() {
        return None;
    }
    let auth_header = auth_header.unwrap();
    let auth_header = auth_header.to_str();
    if auth_header.is_err() {
        return None;
    }
    let auth_header = auth_header.unwrap();

    // hash the auth header
    let mut sha = sha2::Sha512::new();
    sha.update(auth_header.as_bytes());

    let pass = format!("{:02x}", sha.finalize());

    // check if the hash is in the database
    let query = VERIFY_USER;
    let params = vec![serde_json::Value::String(pass.clone())];
    let result = database.query(query, params);

    if result.is_err() {
        return None;
    }

    let result = result.unwrap();
    if result.is_null() {
        return None;
    }
    let result: Result<Vec<crate::routesv1::account::VerifiedAccount>, serde_json::Error> =
        serde_json::from_value(result);

    if result.is_err() {
        return None;
    }
    let result = result.unwrap();
    result.get(0).cloned()
}
