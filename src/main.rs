use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{
    env::{self, VarError},
    error::Error,
};
use teloxide::{
    Bot,
    dispatching::DpHandlerDescription,
    dptree::{self, Handler},
    prelude::{DependencyMap, Dispatcher},
};

mod make;
use make::{make_bot, make_db};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let host: String = env::var("POSTGRES_HOST")?;
    let user: String = env::var("POSTGRES_USER")?;
    let password: String = env::var("POSTGRES_PASSWORD")?;
    let db: String = env::var("POSTGRES_DB")?;

    let url: String = format!("postgres://{}:{}@{}/{}", user, password, host, db);

    let opt: ConnectOptions = ConnectOptions::new(url);

    let db: DatabaseConnection = Database::connect(opt).await?;

    let token: String = env::var("BOT_TOKEN")?;

    let bot: Bot = Bot::new(token);

    let handler: Handler<'static, DependencyMap, Result<(), ()>, DpHandlerDescription> =
        dptree::entry();

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    db.close().await?;

    Ok(())
}
