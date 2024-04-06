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
        // check if already exists
        if self.0.contains(&service) {
            return;
        }
        self.0.push(service);
    }

    pub fn get_service(&self, id: u16) -> Option<Service> {
        // remove reference
        self.0.iter().find(|service| service.id == id).cloned()
    }

    pub fn heartbeat(&mut self) {
        self.0.retain(|service| {
            let heartbeat_request =
                reqwest::blocking::get(format!("http://127.0.0.1:{}/heartbeat", service.id));

            if heartbeat_request.is_err() {
                false
            } else {
                heartbeat_request.unwrap().status().is_success()
            }
        });
    }
}
unsafe impl Send for Services {}
unsafe impl Sync for Services {}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct Workflow(Services);
impl Workflow {
    #![allow(dead_code)]
    pub fn new() -> Self {
        Self(Services::new())
    }

    pub fn add_service(&mut self, service: Service) {
        self.0.add_service(service);
    }

    pub fn remove_service(&mut self, id: u16) {
        self.0 .0.retain(|service| service.id != id);
    }
}
impl Iterator for Workflow {
    type Item = Service;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 .0.pop()
    }
}
unsafe impl Send for Workflow {}
unsafe impl Sync for Workflow {}
