use std::time::Duration;

use axum::extract::ws::{Message, WebSocket};

use crate::error::Errors;

pub struct Port<'a> {
    pub(crate) inner: &'a mut WebSocket,
    pub(crate) closed: &'a mut bool,
}

impl<'a> Port<'a> {
    pub async fn send(&mut self, msg: String) -> Result<(), Errors> {
        let a = tokio::time::timeout(
            Duration::from_secs_f32(0.5),
            self.inner.send(axum::extract::ws::Message::Text(msg)),
        )
        .await;
        let send_result = match a {
            Ok(v) => v,
            Err(_) => {
                *self.closed = true;
                return Err(Errors::WebSocketClosed);
            }
        };

        match send_result {
            Ok(_) => Ok(()),
            Err(err) => {
                *self.closed = true;
                Err(err.into())
            }
        }
    }
    pub async fn receive(&mut self) -> Result<String, Errors> {
        if *self.closed {
            return Err(Errors::WebSocketClosed);
        }
        let a = tokio::time::timeout(Duration::from_secs_f32(0.5), self.inner.recv()).await;
        let received = match a {
            Ok(v) => v,
            Err(_) => {
                *self.closed = true;
                return Err(Errors::WebSocketClosed);
            }
        };
        match received {
            Some(v) => match v {
                Ok(v) => match v {
                    Message::Text(v) => Ok(v),
                    Message::Binary(_) => Err(Errors::WrongMessageType("Binary")),
                    Message::Ping(_) => Err(Errors::WrongMessageType("Ping")),
                    Message::Pong(_) => Err(Errors::WrongMessageType("Pong")),
                    Message::Close(_) => Err(Errors::WrongMessageType("Close")),
                },
                Err(e) => {
                    *self.closed = true;
                    Err(Errors::Axum(e))
                }
            },
            None => {
                *self.closed = true;
                Err(Errors::WebSocketClosed)
            }
        }
    }
}
