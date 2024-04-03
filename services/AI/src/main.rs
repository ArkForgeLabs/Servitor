use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, App, HttpServer};
use ollama_rs::generation::completion::request::GenerationRequest;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub instruction: String,
    pub input: String,
}

#[get("/input_structure")]
async fn input_structure() -> web::Json<Input> {
    web::Json(Input {
        instruction: "string".to_string(),
        input: "string".to_string(),
    })
}

#[post("/input")]
async fn input(
    input: web::Json<Input>,
    ollama: web::Data<ollama_rs::Ollama>,
) -> actix_web::Result<web::Json<serde_json::Value>> {
    let prompt = format!(
        r#"
<|im_start|>system
{}<|im_end|>
<|im_start|>user
{}<|im_end|>
<|im_start|>assistant"#,
        input.instruction, input.input
    );

    let response = ollama
        .generate(GenerationRequest::new(
            "hermes-mistral:instruct".to_string(),
            prompt,
        ))
        .await;

    if response.is_ok() {
        Ok(web::Json(
            serde_json::json!({"response": response.unwrap()}),
        ))
    } else {
        Err(actix_web::error::ErrorInternalServerError(
            response.unwrap_err(),
        ))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::scope("/apiv1").service(input).service(input_structure))
            .app_data(web::Data::new(ollama_rs::Ollama::default()))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
