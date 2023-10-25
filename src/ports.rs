use axum::extract::ws::WebSocket;
use std::collections::HashMap;

use crate::Port;

/// a collection of [Port]
#[derive(Debug)]
pub struct Ports<'a> {
    inner: HashMap<&'a str, &'a mut (bool, WebSocket)>,
}

impl<'a> Ports<'a> {
    pub(crate) fn new(inner: HashMap<&'a str, &'a mut (bool, WebSocket)>) -> Self {
        Self { inner }
    }
}

impl<'a> Ports<'a> {
    pub fn get_port(&mut self, id: &str) -> Option<Port<'a>> {
        let (closed, inner) = self.inner.remove(id)?;
        Some(Port { inner, closed })
    }

    pub fn all_ports(self) -> Vec<(&'a str, Port<'a>)> {
        self.inner
            .into_iter()
            .map(|(k, (closed, ws))| (k, Port { inner: ws, closed }))
            .collect()
    }

    #[allow(unused)]
    fn a(mut self) {
        let a = self.get_port("");
        let b = self.get_port("");
        let c = self.get_port("");

        &a;
        &b;
        &c;
        a;
        b;
        c;
    }
}
