mod at_start;
mod message;

use at_start::{make_bot, make_db};
use message::filter_message;

use sea_orm::DatabaseConnection;
use std::error::Error;
use teloxide::{
    Bot, RequestError,
    dispatching::DpHandlerDescription,
    dptree::{self, Handler},
    prelude::{DependencyMap, Dispatcher},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db: DatabaseConnection = make_db().await?;

    let bot: Bot = make_bot()?;

    let handler: Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> =
        dptree::entry().branch(filter_message());

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![db.clone()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    db.close().await?;

    Ok(())
}
