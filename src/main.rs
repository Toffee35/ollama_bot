use models_manager::ModelsManager;

#[tokio::main]
async fn main() {
    let models_manager: ModelsManager = ModelsManager::new().await.unwrap();
}
