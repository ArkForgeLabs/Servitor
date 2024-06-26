use actix_web::web;
use serde_json::json;
use typeshare::typeshare;

#[typeshare]
pub type OptionalString = Option<String>;
#[typeshare]
pub type OptionalNumber = Option<f32>;

#[typeshare]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq, sqlx::Decode)]
pub struct Connection {
    pub id: String,
    pub source: String,
    pub target: String,
    pub source_output: String,
    pub target_input: String,
}

#[typeshare]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, sqlx::Decode)]
pub struct Control {
    pub name: String,
    #[typeshare(serialized_as = "any")]
    pub value: serde_json::Value,
}

#[typeshare]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct NodeData {
    pub id: String,
    pub label: String,
    pub inputs: Vec<OptionalString>,
    pub outputs: Vec<OptionalString>,
    pub controls: std::collections::HashMap<String, Control>,
    pub position: [OptionalNumber; 2],
    pub connection: Option<Connection>,
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct DBGraph {
    pub id: i32,
    pub content: Vec<NodeData>,
    pub generated_javascript: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
}

pub async fn nodes_graph(
    data: String,
    _app_data: web::Data<crate::AppState>,
) -> actix_web::Result<serde_json::Value> {
    // Attempt to deserialize the JSON value into type T. If this fails, return an error.
    let nodes_list: Vec<NodeData> = serde_json::from_str(&data)?;

    // Attempt to generate the Javascript code from the parsed JSON value
    let code_generated = crate::utils::generate_javascript_code(nodes_list.clone())?;

    // Execute the generated Javascript code using the Deno runtime
    //crate::utils::run_js(code_generated).await;

    Ok(json!({
        "content": nodes_list,
        "generated_javascript": code_generated
    }))
}
