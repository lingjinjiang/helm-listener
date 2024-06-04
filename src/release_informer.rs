use std::{sync::Arc, time::Duration};

use futures::StreamExt;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    runtime::{controller::Action, watcher, Controller},
    Api, Client, ResourceExt,
};
use tokio::sync::mpsc::Sender;

#[derive(thiserror::Error, Debug)]
enum Error {}

#[derive(Clone)]
pub struct SecretListener {
    client: Client,
    sender: Sender<String>,
}

impl SecretListener {
    pub fn new(client: Client, sender: Sender<String>) -> SecretListener {
        SecretListener {
            client: client,
            sender: sender,
        }
    }

    pub async fn start(self) {
        let context = Arc::new(self.clone());
        Controller::new(
            Api::<Secret>::all(self.client.clone()),
            watcher::Config::default().labels("owner=helm"),
        )
        .run(
            SecretListener::reconcile,
            SecretListener::error_policy,
            context,
        )
        .for_each(|_| futures::future::ready(()))
        .await;
    }

    async fn reconcile(secret: Arc<Secret>, ctx: Arc<SecretListener>) -> Result<Action, Error> {
        ctx.sender.send(secret.name_any());
        Ok(Action::await_change())
    }

    fn error_policy(_obj: Arc<Secret>, _error: &Error, _ctx: Arc<SecretListener>) -> Action {
        Action::requeue(Duration::from_secs(60))
    }
}
