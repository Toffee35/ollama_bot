use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{
    env::{self, VarError},
    error::Error,
};
use teloxide::Bot;

pub async fn make_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    let host: String = env::var("POSTGRES_HOST")?;
    let user: String = env::var("POSTGRES_USER")?;
    let password: String = env::var("POSTGRES_PASSWORD")?;

    let url: String = format!("postgres://{}:{}@{}/", user, password, host);

    let opt: ConnectOptions = ConnectOptions::new(url);

    let db: DatabaseConnection = Database::connect(opt).await?;

    Ok(db)
}

pub fn make_bot() -> Result<Bot, VarError> {
    let token: String = env::var("BOT_TOKEN")?;

    Ok(Bot::new(token))
}
