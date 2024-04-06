use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use ollama_rs::generation::completion::request::GenerationRequest;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub instruction: String,
    pub input: String,
}
impl Default for Input {
    fn default() -> Self {
        Self {
            instruction: "string".to_string(),
            input: "string".to_string(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub response: String,
}
impl Default for Output {
    fn default() -> Self {
        Self {
            response: "string".to_string(),
        }
    }
}

#[get("/input_structure")]
async fn input_structure() -> web::Json<Input> {
    web::Json(Input {
        instruction: "string".to_string(),
        input: "string".to_string(),
    })
}

#[get("/output_structure")]
async fn output_structure() -> web::Json<Output> {
    web::Json(Output {
        response: "string".to_string(),
    })
}

#[get("/heartbeat")]
async fn heartbeat() -> impl Responder {
    HttpResponse::Ok()
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
    let response = reqwest::Client::new()
        .post("http://localhost:8080/apiv1/create_service")
        .json(&serde_json::json!({"id": 501, "name": "AI", "description": "AI", "input_structure": Input::default(), "output_structure": Output::default()}))
        .send()
        .await.unwrap();

    println!("Response: {:?}", response);

    HttpServer::new(move || {
        App::new()
            .service(heartbeat)
            .service(web::scope("/apiv1").service(input).service(input_structure))
            .app_data(web::Data::new(ollama_rs::Ollama::default()))
    })
    .bind(("localhost", 501))?
    .run()
    .await?;

    println!("Listening on http://localhost:501");

    Ok(())
}
