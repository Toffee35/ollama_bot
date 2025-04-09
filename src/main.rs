use sea_orm::{Database, DatabaseConnection};
use std::{env, error::Error};
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
    let db: DatabaseConnection = make_db().await?;

    let bot: Bot = make_bot()?;

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
