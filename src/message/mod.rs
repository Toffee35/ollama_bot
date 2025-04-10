use teloxide::{
    RequestError,
    dispatching::{UpdateFilterExt, UpdateHandler},
    types::Update,
};

pub fn filter_message() -> UpdateHandler<RequestError> {
    Update::filter_message().endpoint(message_handler)
}

async fn message_handler() -> Result<(), RequestError> {
    Ok(())
}
