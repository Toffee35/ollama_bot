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

        for container in containers {
            match container.network_settings {
                Some(settings) => match settings.networks {
                    Some(networks) => {
                        let mut ip_addrs: Vec<String> = Vec::new();

                        for (_, parameters) in networks {
                            match parameters.ip_address {
                                Some(ip) => ip_addrs.push(ip),
                                None => continue,
                            }
                        }

                        println!("{:?} -> {:?}", container.names.unwrap(), ip_addrs)
                    }
                    None => continue,
                },
                None => continue,
            }
        }

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
