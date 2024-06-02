use kube::Client;

mod release_informer;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = release_informer::SecretListener::new(Client::try_default().await?);
    listener.start().await;
    Ok(())
}
