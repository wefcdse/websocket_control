use axum::extract::ws::WebSocket;
use std::collections::HashMap;
use tokio::sync::mpsc::{self, error::SendError};

use crate::Ports;

#[derive(Debug, Clone)]
pub struct SocketCollectionHandle {
    sender: mpsc::Sender<(String, WebSocket)>,
}

impl SocketCollectionHandle {
    pub async fn send(
        &self,
        id: String,
        ws: WebSocket,
    ) -> Result<(), SendError<(String, WebSocket)>> {
        self.sender.send((id, ws)).await
    }
}

#[derive(Debug)]
pub struct SocketCollection {
    data: HashMap<String, (bool, WebSocket)>,
    receiver: mpsc::Receiver<(String, WebSocket)>,
    sender: mpsc::Sender<(String, WebSocket)>,
}

impl SocketCollection {
    pub fn new() -> Self {
        let (s, r) = mpsc::channel(1024);

        SocketCollection {
            data: HashMap::new(),
            receiver: r,
            sender: s,
        }
    }
    pub fn get_handle(&self) -> SocketCollectionHandle {
        SocketCollectionHandle {
            sender: self.sender.clone(),
        }
    }
    pub fn collect_connections(&mut self) {
        // dbg!();
        while let Ok((id, ws)) = self.receiver.try_recv() {
            self.data.insert(id, (false, ws));
        }
    }
    pub fn clean(&mut self) {
        self.data = self
            .data
            .drain()
            .filter(|(k, (closed, _ws))| {
                if *closed {
                    log::info!("socket {} closed", k);
                }
                !*closed
            })
            .collect();
    }
    pub fn ports(&mut self) -> Ports<'_> {
        Ports::new(
            self.data
                .iter_mut()
                .map(|(k, v)| -> (&str, &mut (bool, WebSocket)) { (k, v) })
                .collect::<HashMap<_, _>>(),
        )
    }
}
