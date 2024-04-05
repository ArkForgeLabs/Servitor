#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Service {
    /// port of the service
    pub id: u16,
    pub name: String,
    pub input_structure: serde_json::Value,
    pub output_structure: serde_json::Value,
}
impl Service {
    #![allow(dead_code)]
    pub fn new(
        id: u16,
        name: String,
        input_structure: serde_json::Value,
        output_structure: serde_json::Value,
    ) -> Self {
        Self {
            id,
            name,
            input_structure,
            output_structure,
        }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}
unsafe impl Send for Service {}
unsafe impl Sync for Service {}
