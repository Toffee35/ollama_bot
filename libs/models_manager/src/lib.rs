use bollard::{Docker, errors::Error, models::ContainerSummary};
use ollama_rs::Ollama;
use std::collections::HashMap;

pub struct ModelsManager {
    connection: Docker,
    models: HashMap<String, HashMap<String, Ollama>>,
}

impl ModelsManager {
    pub async fn new() -> Result<Self, Error> {
        let connection: Docker = Docker::connect_with_socket_defaults()?;

        let containers: Vec<ContainerSummary> = connection.list_containers::<String>(None).await?;

        println!("{:#?}", containers);

        todo!()
    }

    pub fn get(&self) -> Vec<String> {
        todo!()
    }

    pub fn get_variant(&self, name: String) -> Vec<String> {
        todo!()
    }

    pub fn generate(&self, name: String, variant: String) -> String {
        todo!()
    }
}
