use teloxide::types::ChatId;

pub enum State {
    Base,
}

pub struct ChatsState {
    id: ChatId,
    state: State,
}
