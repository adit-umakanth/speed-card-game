use std::{
    collections::HashMap,
    sync::{Arc, Mutex}, error::Error,
};

use futures_util::{SinkExt, StreamExt};
use log::info;
use pretty_env_logger;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

type WaitingRoom = Arc<Mutex<HashMap<String, WebSocket>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let waiting_room = WaitingRoom::default();
    let waiting_room = warp::any().map(move || waiting_room.clone());

    let speed = warp::path("speed").and(warp::ws()).and(waiting_room).map(
        |ws: warp::ws::Ws, waiting_room| {
            ws.on_upgrade(move |socket| handle_connection(socket, waiting_room))
        },
    );

    warp::serve(speed).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_connection(ws: WebSocket, waiting_room: WaitingRoom) {
    match user_connected(ws, waiting_room).await {
        Ok(_) => log::info!("Closed successfully!"),
        Err(e) => log::error!("Something happened: {}", e),
    }
}

async fn user_connected(mut ws: WebSocket, waiting_room: WaitingRoom) -> Result<(), Box<dyn Error>> {
    log::info!("Connected with client");

    let message = ws.next().await;
    let msg: String;

    if let Some(Ok(message)) = message {
        info!("Raw message: {:#?}", message);
        msg = message.to_str().map_err(|_| format!("Error converting message to string"))?.to_owned();
    } else {
        return Err("No room message received".into());
    }

    let websockets = handle_waiting_room(msg, ws, waiting_room);
    match websockets {
        Some(websockets) => {
            let (mut player1_ws, mut player2_ws) = websockets;
            player1_ws.send(Message::text("Wow!")).await?;
            player2_ws.send(Message::text("Wow!")).await?;
        },
        None => (),
    }

    Ok(())
}

fn handle_waiting_room(
    room_name: String,
    ws: WebSocket,
    waiting_room: WaitingRoom,
) -> Option<(WebSocket, WebSocket)> {
    let mut guard = waiting_room.lock().unwrap();
    match guard.remove(&room_name) {
        Some(player2_ws) => Some((ws, player2_ws)),
        None => {
            guard.insert(room_name, ws);
            None
        },
    }
}
