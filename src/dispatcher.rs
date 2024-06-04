use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc::Receiver;

pub struct Dispatcher {
    receiver: Receiver<String>,
    handlres: HashMap<String, Arc<Mutex<dyn FnMut(String)>>>,
}

pub fn new(receiver: Receiver<String>) -> Dispatcher {
    Dispatcher {
        receiver: receiver,
        handlres: HashMap::new(),
    }
}

impl Dispatcher {
    pub async fn run(&mut self) {
        let handlers = self.handlres.clone();
        loop {
            tokio::select! {
                Some(msg) = self.receiver.recv() => {
                        (handlers["test"].lock().unwrap())(msg);
                }
            }
        }
    }

    pub fn register(&mut self, handler: impl FnMut(String) + 'static) {
        self.handlres
            .insert("test".to_string(), Arc::new(Mutex::new(handler)));
    }
}
