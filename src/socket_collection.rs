use axum::extract::ws::WebSocket;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicIsize},
        Arc,
    },
};
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

#[derive(Debug, Clone)]
pub struct SocketCollectionStateHandle {
    pub ws_count: Arc<AtomicIsize>,
    pub ws_added: Arc<AtomicBool>,
}

#[derive(Debug)]
pub struct SocketCollection {
    data: HashMap<String, (bool, WebSocket)>,
    ws_count: Arc<AtomicIsize>,
    ws_added: Arc<AtomicBool>,
    receiver: mpsc::Receiver<(String, WebSocket)>,
    sender: mpsc::Sender<(String, WebSocket)>,
}

impl SocketCollection {
    pub fn new() -> Self {
        let (s, r) = mpsc::channel(1024);

        SocketCollection {
            data: HashMap::new(),
            ws_count: Arc::new(AtomicIsize::new(0)),
            ws_added: Arc::new(AtomicBool::new(false)),
            receiver: r,
            sender: s,
        }
    }
    pub fn get_handle(&self) -> SocketCollectionHandle {
        SocketCollectionHandle {
            sender: self.sender.clone(),
        }
    }
    pub fn get_state_handle(&self) -> SocketCollectionStateHandle {
        SocketCollectionStateHandle {
            ws_count: self.ws_count.clone(),
            ws_added: self.ws_added.clone(),
        }
    }
    pub fn collect_connections(&mut self) {
        // dbg!();
        while let Ok((id, ws)) = self.receiver.try_recv() {
            let add_new = self.data.insert(id, (false, ws)).is_none();
            if add_new {
                self.ws_added
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                self.ws_count.store(
                    self.data.len() as isize,
                    std::sync::atomic::Ordering::Relaxed,
                );
            }
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
        self.ws_count.store(
            self.data.len() as isize,
            std::sync::atomic::Ordering::Relaxed,
        );
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
