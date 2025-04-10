use teloxide::{
    RequestError,
    dispatching::{DpHandlerDescription, UpdateFilterExt},
    prelude::{DependencyMap, Handler},
    types::Update,
};

pub fn filter_message()
-> Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> {
    Update::filter_message().endpoint(message_handler)
}

async fn message_handler() -> Result<(), RequestError> {
    Ok(())
}
