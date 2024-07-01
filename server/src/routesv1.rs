use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use serde::de::Error;
use sqlx::Row;
use utils::Service;

pub mod account;
pub mod nodes;

#[get("/get_service/{service_id}")]
pub async fn get_service(
    service_id: web::Path<u16>,
    app_data: web::Data<crate::AppState>,
) -> actix_web::Result<Json<Service>> {
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
    service: Json<Service>,
    app_data: web::Data<crate::AppState>,
) -> impl Responder {
    let mut services = app_data.services.lock().unwrap();
    services.add_service(service.into_inner());

    HttpResponse::Ok()
}

#[post("/{service}/create")]
pub async fn create(
    service: web::Path<String>,
    data: String,
    app_data: web::Data<crate::AppState>,
) -> actix_web::Result<HttpResponse> {
    // This function handles the POST request for creating a new resource in a specified service.

    // Match the service name and call the typecheck function with the appropriate struct type as a generic argument.
    // If the service name is invalid, return an error.
    let result: actix_web::Result<serde_json::Value> = match service.as_str() {
        "users" => {
            // Parse the input JSON string into a serde_json::Value object
            let mut json_form: serde_json::Value = serde_json::from_str(&data)?;

            // Attempt to deserialize the JSON value into type T. If this fails, return an error.
            serde_json::from_value::<account::Account>(json_form.clone())?;

            json_form.as_object_mut().unwrap().remove("id");

            // Check if the parsed JSON value is an object. If it is not, return an error.
            if json_form.as_object().is_some() {
                Ok(json_form)
            } else {
                Err(actix_web::error::ErrorBadRequest(
                    serde_json::Error::custom("Invalid JSON"),
                ))
            }
        }

        "graph" => nodes::nodes_graph(data, app_data.clone()).await,

        _ => Err(actix_web::error::ErrorBadRequest(
            serde_json::Error::custom("Invalid JSON"),
        )),
    };

    // If the typecheck function returned an error, return a 404 Not Found response.
    // Otherwise, continue with the creation process.
    if result.is_err() {
        return Err(actix_web::error::ErrorNotFound("Service not found"));
    }

    let result = result.unwrap();
    let result = result.as_object().unwrap();

    // Extract the keys and values from the JSON object and format them into a SQL query string.
    let keys = result.keys().collect::<Vec<_>>();
    let values = result.values().collect::<Vec<_>>();

    let query = format!(
        "INSERT INTO {} ({}, creation_date) VALUES ({}, CURRENT_TIMESTAMP)",
        service,
        keys.iter()
            .map(|key| key.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        {
            let mut counter = 1;
            keys.iter()
                .map(|_| {
                    counter += 1;
                    format!("${}", counter - 1)
                })
                .collect::<Vec<_>>()
                .join(", ")
        }
    );

    // Prepare the SQL query with the extracted values and execute it using the database connection pool.
    let mut prepared_query = sqlx::query(&query);
    for i in values {
        if i.is_string() {
            prepared_query = prepared_query.bind(i.as_str().unwrap());
        } else if i.is_number() {
            prepared_query = prepared_query.bind(i.as_i64().unwrap());
        } else if i.is_boolean() {
            prepared_query = prepared_query.bind(i.as_bool().unwrap());
        } else if i.is_object() {
            prepared_query = prepared_query.bind(i);
        } else if i.is_array() {
            prepared_query = prepared_query.bind(i.as_array().unwrap());
        } else {
            return Err(actix_web::error::ErrorInternalServerError(
                "Failed to bind value",
            ));
        }
    }

    // If the query execution was successful, return a 200 OK response. Otherwise, return an error.
    match prepared_query.execute(&app_data.database.pool).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => {
            println!("Error: {}", e);
            return Err(actix_web::error::ErrorInternalServerError(
                "Failed to execute query",
            ));
        }
    }
}

#[get("/{service}/get_all")]
pub async fn get_all(
    service: web::Path<String>,
    app_data: web::Data<crate::AppState>,
) -> actix_web::Result<HttpResponse> {
    match service.as_str() {
        "users" => {
            let _rows = sqlx::query("SELECT * FROM users")
                .fetch_all(&app_data.database.pool)
                .await
                .expect("Failed to execute query")
                .iter()
                .map(|row| {
                    println!(
                        "\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
                        row.get::<i32, _>(0),
                        row.get::<String, _>(1),
                        row.get::<String, _>(2),
                        row.get::<String, _>(3),
                        row.get::<Option<String>, _>(4),
                        row.get::<chrono::DateTime<chrono::Utc>, _>(5),
                    );
                })
                .collect::<()>();
        }
        "graph" => {
            let rows = sqlx::query("SELECT * FROM graph")
                .fetch_all(&app_data.database.pool)
                .await
                .expect("Failed to execute query")
                .iter()
                .map(|row| {
                    let content: Vec<serde_json::Value> = row.get(1);

                    nodes::DBGraph {
                        id: row.get(0),
                        content: content
                            .iter()
                            .map(|value| {
                                serde_json::from_value::<nodes::NodeData>(value.clone())
                                    .expect("Failed to parse JSON")
                            })
                            .collect::<Vec<nodes::NodeData>>(),

                        generated_javascript: row.get(2),
                        creation_date: row.get(3),
                    }
                })
                .collect::<Vec<nodes::DBGraph>>();

            println!("{rows:?}");
        }
        _ => return Err(actix_web::error::ErrorNotFound("Service not found")),
    }

    Ok(HttpResponse::Ok().finish())
}
