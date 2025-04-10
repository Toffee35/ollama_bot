use teloxide::types::ChatId;

pub enum CreatorIs {
    User,
    Model,
}

pub struct ChatMessages {
    chat_id: ChatId,
    position: u64,
    creator: CreatorIs,
    text: String,
}
