use teloxide::types::ChatId;

pub enum State {
    Base,
}

pub struct ChatsState {
    id: i64,
    state: String,
}
