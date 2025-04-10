mod tables;

pub use tables::{ChatMessages, ChatsState, CreatorIs, State, create_tables};

pub async fn make_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    let host: String = env::var("POSTGRES_HOST")?;
    let user: String = env::var("POSTGRES_USER")?;
    let password: String = env::var("POSTGRES_PASSWORD")?;
    let db_name: String = env::var("POSTGRES_DB")?;

    let url: String = format!("postgres://{}:{}@{}/{}", user, password, host, db_name);

    let opt: ConnectOptions = ConnectOptions::new(url);

    let db: DatabaseConnection = Database::connect(opt).await?;

    create_tables(&db);

    Ok(db)
}
