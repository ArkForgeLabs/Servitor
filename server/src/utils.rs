use actix_web::HttpRequest;
use sha2::Digest;

// SQL Queries
// user
pub const CREATE_USER: &str = "INSERT INTO users (username, email, password) VALUES (?, ?, ?)";
pub const DELETE_USER: &str = "DELETE FROM users WHERE id = ?";
pub const UPDATE_USER: &str = "UPDATE users SET username = ?, email = ? WHERE id = ?";
pub const VERIFY_USER: &str = "SELECT id, username, email FROM users WHERE password = ?";
// nodes
//pub const CREATE_NODE: &str = "INSERT INTO nodes (id, label, inputs, outputs, controls, position, connection) VALUES (?, ?, ?, ?, ?, ?, ?)";

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
    println!("service: {service_name} | data: {data}");

    let result = reqwest::blocking::get("https://myip.wtf/json");

    match result {
        Ok(value) => println!("{:?}", value.text()),
        Err(e) => println!("{:?}", e),
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
            r#"
// Print helper function, calling Deno.core.print()
function print(value) {
Deno.core.print(value.toString()+"\n");
}

// helper function to call services
function call_service(service_name, data){
Deno.core.ops.call_service(service_name, JSON.stringify(data));
}"#,
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

pub fn generate_javascript_code(nodes_list: Vec<crate::routesv1::nodes::NodeData>) -> String {
    fn walk_tree(found_node: &mut NodeTreeItem, node: &NodeTreeItem, parent_id: &String) {
        if &found_node.id == parent_id {
            found_node.children.push(node.clone())
        } else {
            for child in found_node.children.iter_mut() {
                walk_tree(child, node, parent_id);
            }
        }
    }

    fn walk_tree_and_generate_code(node: &NodeTreeItem, output: &mut String) {
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

            "Math" => {
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
            _ => {}
        }

        output.clear();
        output.insert_str(0, &new_output);

        node.children.iter().for_each(|child_node| {
            walk_tree_and_generate_code(child_node, output);
        });
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

    let mut root = nodes[0].clone();

    for node in nodes.iter() {
        if node.connection.is_some() {
            let parent_id = &node.connection.as_ref().unwrap().target;

            walk_tree(&mut root, node, parent_id);
        }
    }

    let mut output = format!("print(var{});\n", root.id);
    walk_tree_and_generate_code(&root, &mut output);

    output = "\n".to_string() + &output;

    "\n".to_string() + &output
}
