use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    response::IntoResponse,
};

use futures::StreamExt;
use tokio::sync::broadcast;

use crate::builder::BUILD_CHANNEL;

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {

    let mut rx;

    unsafe {

        rx = BUILD_CHANNEL.as_ref().unwrap().subscribe();
    }

    while let Ok(msg) = rx.recv().await {

        if socket.send(Message::Text(msg)).await.is_err() {

            return;
        }
    }
}