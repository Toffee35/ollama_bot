use std::env::{self, VarError};
use teloxide::Bot;

pub fn make_bot() -> Result<Bot, VarError> {
    let token: String = env::var("BOT_TOKEN")?;

    Ok(Bot::new(token))
}
