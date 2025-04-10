mod chat_messages;
mod chats_state;

pub use chat_messages::{ChatMessages, CreatorIs};
pub use chats_state::{ChatsState, State};

pub fn create_tables(db: &DatabaseConnection) {}
