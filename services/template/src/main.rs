use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use utils::Service;

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
async fn input(_input: web::Json<Input>) -> actix_web::Result<web::Json<serde_json::Value>> {
    /* implementation */

    Ok(web::Json(serde_json::json!({"response": _input.input})))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let response = reqwest::Client::new()
        .post("http://localhost:8080/apiv1/create_service")
        .json(
            &serde_json::to_string(&Service::new(
                501,
                "template".to_string(),
                "template".to_string(),
                serde_json::to_value(Input::default())?,
                serde_json::to_value(Output::default())?,
            ))?
            .to_string(),
        )
        .send()
        .await.unwrap();

    println!("Response: {:?}", response);

    HttpServer::new(move || {
        App::new()
            .service(heartbeat)
            .service(web::scope("/apiv1").service(input).service(input_structure))
    })
    .bind(("localhost", 501))?
    .run()
    .await?;

    println!("Listening on http://localhost:501");

    Ok(())
}
