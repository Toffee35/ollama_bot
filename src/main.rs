use sea_orm::{Database, DatabaseConnection};
use std::{env, error::Error};
use teloxide::{
    Bot,
    dispatching::DpHandlerDescription,
    dptree::{self, Handler},
    prelude::{DependencyMap, Dispatcher},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let host: String = env::var("POSTGRES_HOST")?;
    let user: String = env::var("POSTGRES_USER")?;
    let password: String = env::var("POSTGRES_PASSWORD")?;

    let opt: String = format!(
        "postgres://{}:{}@{}/database?currentSchema=my_schema",
        user, password, host
    );

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

    Ok(())
}
