use kube::Client;
use tokio::sync::mpsc;

mod dispatcher;
mod handler;
mod release_informer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel::<String>(10);
    let mut dispatcher = dispatcher::new(receiver);
    let listener = release_informer::SecretListener::new(Client::try_default().await?, sender);
    tokio::task::spawn(async move { listener.start().await });
    dispatcher.register(handler::handle_create);
    dispatcher.run().await;
    Ok(())
}
