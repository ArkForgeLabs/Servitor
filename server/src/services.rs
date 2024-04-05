#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct Service {
    /// port of the service
    pub id: u16,
    pub name: String,
    pub description: String,
    pub input_structure: serde_json::Value,
    pub output_structure: serde_json::Value,
}
impl Service {
    #![allow(dead_code)]
    pub fn new(
        id: u16,
        name: String,
        description: String,
        input_structure: serde_json::Value,
        output_structure: serde_json::Value,
    ) -> Self {
        Self {
            id,
            name,
            description,
            input_structure,
            output_structure,
        }
    }
}
unsafe impl Send for Service {}
unsafe impl Sync for Service {}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct Services(Vec<Service>);
impl Services {
    #![allow(dead_code)]
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add_service(&mut self, service: Service) {
        self.0.push(service);
    }

    pub fn get_service(&self, id: u16) -> Option<Service> {
        // remove reference
        self.0.iter().find(|service| service.id == id).cloned()
    }
}
unsafe impl Send for Services {}
unsafe impl Sync for Services {}
