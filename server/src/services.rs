use utils::Service;

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
