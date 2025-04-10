use sea_orm::DatabaseConnection;

mod chat_messages;
mod chats_state;

pub use chat_messages::{ChatMessages, Creator};
pub use chats_state::{ChatsState, State};

pub fn create_tables(db: &DatabaseConnection) {
    todo!()
}
