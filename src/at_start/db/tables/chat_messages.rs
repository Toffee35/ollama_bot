use teloxide::types::ChatId;

pub enum Creator {
    User,
    Model,
}

pub struct ChatMessages {
    chat_id: i64,
    position: i64,
    creator: String,
    text: String,
}
