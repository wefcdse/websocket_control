use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    headers,
    response::IntoResponse,
    routing::get,
    Router, TypedHeader,
};
use futures::Future;

use crate::{SocketCollection, SocketCollectionHandle};

pub fn get_router<F, Fut>(main: F) -> Router
where
    F: FnOnce(SocketCollection) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let sc = SocketCollection::new();
    let handle = sc.get_handle();
    tokio::spawn(main(sc));
    Router::new()
        //绑定websocket路由
        .route("/ws", get(ws_handler).with_state(handle))
}

pub async fn ws_handler(
    State(socket_collection): State<SocketCollectionHandle>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = &user_agent {
        log::trace!("useragemt: `{}` connected", user_agent.as_str());
    }
    log::trace!("new connect");
    // dbg!(user_agent);

    ws.on_upgrade(|socket: WebSocket| handle_new_websocket(socket_collection, socket))
}

async fn handle_new_websocket(socket_collection: SocketCollectionHandle, mut socket: WebSocket) {
    // let defer = Defer::new(|| println!("closed"));
    loop {
        let msg = if let Some(Ok(v)) = socket.recv().await {
            v
        } else {
            return;
        };

        // dbg!(&msg);
        match msg {
            axum::extract::ws::Message::Text(text) => {
                let splited = text.split(' ').collect::<Vec<_>>();
                if let Some(&"id") = splited.get(0) {
                    // println!("{}", text);
                    if let Some(id) = splited.get(1) {
                        log::info!("port {} connected", id);
                        let id = id.to_owned().to_owned();
                        let _ = socket_collection.send(id, socket).await;
                        return;
                    }
                    // store socket here
                    return;
                } else {
                    return;
                }
            }
            axum::extract::ws::Message::Binary(_) => return,
            axum::extract::ws::Message::Close(_) => return,
            axum::extract::ws::Message::Ping(_) => continue,
            axum::extract::ws::Message::Pong(_) => continue,
        }
    }
}
