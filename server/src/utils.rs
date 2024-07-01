use actix_web::HttpRequest;
use sha2::Digest;

// SQL Queries
pub const DELETE_USER: &str = "DELETE FROM users WHERE id = ?";
pub const UPDATE_USER: &str = "UPDATE users SET username = ?, email = ? WHERE id = ?";
pub const VERIFY_USER: &str = "SELECT id, username, email FROM users WHERE password = ?";

pub fn verify(
    database: &crate::database::Database,
    request: HttpRequest,
) -> actix_web::Result<crate::routesv1::account::VerifiedAccount> {
    // get the auth header
    let auth_header = request.headers().get("Authorization");
    if auth_header.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    let auth_header = auth_header.unwrap();
    let auth_header = auth_header.to_str();
    if auth_header.is_err() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    let auth_header = auth_header.unwrap();
    let pass = hash(auth_header);

    // check if the hash is in the database
    let query = VERIFY_USER;
    let params = vec![serde_json::Value::String(pass.clone())];
    let result = database.query(query, params);

    if result.is_err() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    let result = result.unwrap();
    if result.is_null() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    let result: Result<Vec<crate::routesv1::account::VerifiedAccount>, serde_json::Error> =
        serde_json::from_value(result);

    if result.is_err() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    let result = result.unwrap();
    let first_result = result.first();

    if first_result.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }

    Ok(first_result.unwrap().clone())
}

pub fn hash(password: &str) -> String {
    let mut sha = sha2::Sha512::new();
    sha.update(password.as_bytes());
    format!["{:02x}", sha.finalize()]
}

#[deno_core::op2(fast)]
fn call_service(
    #[string] service_name: String,
    #[string] data: String,
) -> Result<(), deno_core::anyhow::Error> {
    println!("\n\nservice: {service_name} | data: {data}\n\n");

    match service_name.as_str() {
        "http_request" => {
            let parsed_data = serde_json::from_str::<serde_json::Value>(&data).unwrap();
            let url = parsed_data["url"].as_str().unwrap();
            let content = serde_json::to_string(&parsed_data["content"]).unwrap();
            let method = parsed_data["method"].as_str().unwrap();
            let content_type = parsed_data["content type"].as_str().unwrap();

            let client = reqwest::blocking::Client::new();
            let request_builder;
            match method {
                "get" => request_builder = client.get(url),
                "post" => request_builder = client.post(url),
                "put" => request_builder = client.put(url),
                "delete" => request_builder = client.delete(url),
                _ => {
                    return Err(deno_core::anyhow::Error::msg("invalid method"));
                }
            }

            let request = request_builder
                .header("content-type", content_type)
                .body(content)
                .build()?;
            let result = client.execute(request)?;

            println!("result: {result:?}");
        }
        _ => {}
    }

    // return as a Result<f64, AnyError>
    Ok(())
}

#[allow(dead_code)]
pub async fn run_js(code: String) {
    actix_web::web::block(move || {
        // Build a deno_core::Extension providing custom ops
        const CALL: deno_core::OpDecl = call_service();
        let ext = deno_core::Extension {
            name: "my_ext",
            ops: std::borrow::Cow::Borrowed(&[CALL]),
            ..Default::default()
        };

        // Initialize a runtime instance
        let mut runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            extensions: vec![ext],
            ..Default::default()
        });

        // Now we see how to invoke the op we just defined. The runtime automatically
        // contains a Deno.core object with several functions for interacting with it.
        // You can find its definition in core.js.

        let final_bundled_code = format!(
            "{}{}",
            r#"// ===== SERVITOR CODE ===== //
function print(value) {
  Deno.core.print(value.toString()+"\n");
}

function call_service(service_name, data){
  Deno.core.ops.call_service(service_name, JSON.stringify(data));
}

// ===== GRAPH CODE ===== //"#,
            code
        );

        println!("{final_bundled_code}");

        runtime
            .execute_script("<usage>", final_bundled_code)
            .unwrap();
    })
    .await
    .unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeTreeItem {
    pub id: String,
    pub label: String,
    pub controls: std::collections::HashMap<String, crate::routesv1::nodes::Control>,
    pub connection: Option<crate::routesv1::nodes::Connection>,
    pub children: Vec<NodeTreeItem>,
}

pub fn generate_javascript_code(
    nodes_list: Vec<crate::routesv1::nodes::NodeData>,
) -> actix_web::Result<String> {
    fn walk_tree(found_node: &mut NodeTreeItem, node: &NodeTreeItem, parent_id: &String) {
        if &found_node.id == parent_id {
            found_node.children.push(node.clone())
        } else {
            for child in found_node.children.iter_mut() {
                walk_tree(child, node, parent_id);
            }
        }
    }

    fn walk_tree_and_generate_code(
        node: &NodeTreeItem,
        output: &mut String,
    ) -> actix_web::Result<()> {
        let mut new_output = output.clone();

        let children_ids = node
            .children
            .iter()
            .map(|node| &node.id)
            .collect::<Vec<&String>>();

        match node.label.as_str() {
            "Number" => {
                new_output = format!(
                    "let var{} = {};\n",
                    node.id,
                    node.controls.get("value").unwrap().value.as_f64().unwrap()
                ) + &new_output
            }

            "String" => {
                new_output = format!(
                    "let var{} = \"{}\";\n",
                    node.id,
                    node.controls.get("value").unwrap().value.as_str().unwrap()
                ) + &new_output
            }

            "Math" => {
                if children_ids.len() != 2 {
                    return Err(actix_web::error::ErrorBadRequest(format!(
                        "Invalid number of children for Math node: {children_ids:?}"
                    )));
                }
                let operations = ["+", "-", "*", "/"];
                let operation_index = &node
                    .controls
                    .get("operation")
                    .unwrap()
                    .value
                    .as_u64()
                    .unwrap();
                let operation = operations.get(*operation_index as usize).unwrap_or(&&"+");

                new_output = format!(
                    "let var{} = var{} {} var{};\n",
                    node.id, children_ids[0], operation, children_ids[1]
                ) + &new_output
            }

            "Table" => {
                new_output = format!(
                    "let var{} = {};\n",
                    node.id,
                    serde_json::to_string(&node.controls.get("data").unwrap().value)?
                ) + &new_output
            }

            "HTTP Request" => {
                if children_ids.len() != 2 {
                    return Err(actix_web::error::ErrorBadRequest(format!(
                        "Invalid number of children for HTTP Request node: {children_ids:?}"
                    )));
                }

                let methods = ["get", "post", "put", "delete"];
                let content_types = ["application/json", "text/plain"];

                let (url, content) = {
                    if node.children[0].label == "String" {
                        (children_ids[0], children_ids[1])
                    } else {
                        (children_ids[1], children_ids[0])
                    }
                };

                new_output = format!(
                    "let var{} = call_service(\"http_request\", {{ \"url\": var{}, \"content\": var{}, \"method\": \"{}\", \"content type\": \"{}\" }});",
                    node.id,
                    url,
                    content,
                    methods.get(node.controls.get("method").unwrap().value.as_u64().unwrap() as usize).unwrap_or(&"get"),
                    content_types.get(node.controls
                        .get("content type")
                        .unwrap()
                        .value
                        .as_u64()
                        .unwrap() as usize).unwrap_or(&"text/plain"),
                )
            }
            _ => {}
        }

        output.clear();
        output.insert_str(0, &new_output);

        node.children.iter().for_each(|child_node| {
            walk_tree_and_generate_code(child_node, output).expect("Code Generation Failed");
        });

        Ok(())
    }

    let nodes: Vec<NodeTreeItem> = nodes_list
        .iter()
        .map(|node| NodeTreeItem {
            id: node.id.clone(),
            label: node.label.clone(),
            controls: node.controls.clone(),
            connection: node.connection.clone(),
            children: Vec::new(),
        })
        .collect();

    // find index of the node without connection
    let mut root = nodes
        .iter()
        .filter(|n| n.connection.is_none())
        .next()
        .unwrap()
        .clone();

    for node in nodes.iter() {
        if node.connection.is_some() {
            let parent_id = &node.connection.as_ref().unwrap().target;

            walk_tree(&mut root, node, parent_id);
        }
    }

    let mut output = format!("print(var{});\n", root.id);
    walk_tree_and_generate_code(&root, &mut output)?;

    output = "\n".to_string() + &output;

    Ok("\n".to_string() + &output)
}
