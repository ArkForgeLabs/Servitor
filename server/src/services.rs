use utils::Service;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct Services(Vec<Service>);
impl Services {
    #![allow(dead_code)]
    pub async fn new() -> Self {
        let docker_file =
            serde_yaml::from_str::<serde_yaml::Value>(include_str!("../../docker-compose.yml"));

        if docker_file.is_err() {
            return Self(vec![]);
        }
        let docker_file = docker_file.unwrap();
        let docker_file = docker_file.as_mapping().unwrap()["services"]
            .as_mapping()
            .unwrap();

        let services_to_ignore = ["db"];

        let mut services = Vec::new();

        for service_name in docker_file.keys() {
            // if the service_name is not in services_to_ignore
            if services_to_ignore.contains(&service_name.as_str().unwrap()) {
                continue;
            }

            let service = docker_file[service_name].as_mapping().unwrap();
            let port = service["ports"][0]
                .as_str()
                .unwrap()
                .split(":")
                .collect::<Vec<&str>>()[0];
            let port = port.parse::<u16>().unwrap_or(0);

            let info_response = reqwest::get(format!("http://localhost:{}/info", port)).await;
            if info_response.is_err() {
                continue;
            }
            let service_info: serde_json::Value = info_response.unwrap().json().await.unwrap();

            services.push(Service {
                id: port,
                name: service_name.as_str().unwrap().to_string(),
                description: service_info["description"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                version: service_info["version"].as_str().unwrap_or("").to_string(),
                input_structure: service_info["input_structure"].clone(),
                output_structure: service_info["output_structure"].clone(),
            });
        }

        Self(services)
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
    pub async fn new() -> Self {
        Self(Services::new().await)
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
