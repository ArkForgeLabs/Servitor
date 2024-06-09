use typeshare::typeshare;

#[typeshare]
pub type OptionalString = Option<String>;
#[typeshare]
pub type OptionalNumber = Option<i32>;

#[typeshare]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct Connection {
    pub id: String,
    pub source: String,
    pub target: String,
    pub source_output: String,
    pub target_input: String,
}

#[typeshare]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct Control {
    pub name: String,
    pub value: String,
}

#[typeshare]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct NodeData {
    pub id: String,
    pub label: String,
    pub inputs: Vec<OptionalString>,
    pub outputs: Vec<OptionalString>,
    pub controls: std::collections::HashMap<String, Control>,
    pub position: [OptionalNumber; 2],
    pub connection: Option<Connection>,
}
