use std::{net::SocketAddr, u8};

use axum::{
    extract::{
        ws::{self, WebSocket},
        ConnectInfo, Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};

use yrs::{
    encoding::{
        read::{Cursor, Read},
        write::Write,
    }, updates::{
        decoder::{Decode, DecoderV1},
        encoder::{Encode, Encoder, EncoderV1},
    }, Doc, ReadTxn, StateVector, Transact, Update
};

use crate::AppState;

enum Message {
    MessageSync,
    MessageAwareness,
    NULL,
}

impl Message {
    fn to_u8(self) -> u8 {
        match self {
            Message::MessageSync => 0,
            Message::MessageAwareness => 1,
            Message::NULL => 255,
        }
    }

    fn from_u8(value: u8) -> Self {
        match value {
            0 => Message::MessageSync,
            1 => Message::MessageAwareness,
            _ => Message::NULL,
        }
    }
}

enum YrsMessage {
    MessageYjsSyncStep1,
    MessageYjsSyncStep2,
    MessageYjsUpdate,
    NULL,
}

impl YrsMessage {
    fn to_u8(self) -> u8 {
        match self {
            YrsMessage::MessageYjsSyncStep1 => 0,
            YrsMessage::MessageYjsSyncStep2 => 1,
            YrsMessage::MessageYjsUpdate => 2,
            YrsMessage::NULL => 255,
        }
    }

    fn from_u8(value: u8) -> Self {
        match value {
            0 => YrsMessage::MessageYjsSyncStep1,
            1 => YrsMessage::MessageYjsSyncStep2,
            2 => YrsMessage::MessageYjsUpdate,
            _ => YrsMessage::NULL,
        }
    }
}



pub async fn ws_handler(
    State(app_state): State<AppState>,
    Path(room): Path<String>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let doc = {
        let docs = &app_state.docs.read().unwrap();
        let doc = docs.get(&room);
        match doc {
            Some(doc) => doc.clone(),
            None => yrs::Doc::new(),
        }
    };

    {
        // println!("Creating new doc for room `{}`", room);
        app_state.docs.write().unwrap().insert(room.clone(), doc);
    }

    // println!("`{}` connected.", room);

    let doc = {
        let docs = &app_state.docs.read().unwrap();
        docs.get(&room).unwrap().clone()
    };

    // {
    //     // list output of docs
    //     let docs = &app_state.read().unwrap().docs;
    //     for (key, value) in docs.iter() {
    //         let txn = value.transact();
    //         if room == "notfly-collection" {
    //             let metas = txn.get_map("meta");
    //             if let Some(metas) =  metas {
    //                 println!("key: {}, value: {:?}", key, metas.to_json(&txn));
    //             }
    //         }else {
    //             let blocks = txn.get_map("blocks");
    //             if let Some(blocks) = blocks {
    //                 let blocks = blocks.to_json(&txn);
    //                 println!("key: {}, value: {:?}", key, blocks);
    //             }
    //         }
    //     }
    // }

    ws.on_upgrade(move |socket| handle_socket(socket, addr, room, doc))
}

async fn handle_socket(mut socket: WebSocket, _who: SocketAddr, _room:String, doc: Doc) {
    {
        // Send sync step 1
        let mut encoder = EncoderV1::new();
        encoder.write_u8(Message::MessageSync.to_u8());
        encoder.write_u8(YrsMessage::MessageYjsSyncStep1.to_u8());
        let sv = doc.transact_mut().state_vector().encode_v1();
        encoder.write_len(sv.len() as u32);
        encoder.write_all(&sv);
        let message = encoder.to_vec();
        // println!("Sending MessageYjsSyncStep1 {:?}", message);
        socket.send(ws::Message::Binary(message)).await.unwrap();
    }

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<bool>();

    // interval send awareness message
    let ping_spawn = tokio::spawn(async move {
        loop {
            tx.send(true).unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    });

    loop {
        tokio::select! {
            msg = socket.recv() => {
                let msg = if let Some(msg) = msg{
                    match msg {
                        Ok(msg) => Some(msg),
                        Err(_) => None
                    }
                }else{
                    None
                };
                match msg {
                    Some(ws::Message::Binary(message)) => {
                        let mut encoder = EncoderV1::new();
                        let mut decoder = DecoderV1::new(Cursor::new(&message));
                        let message_type = decoder.read_u8().unwrap();
                        match Message::from_u8(message_type) {
                            Message::MessageSync => {
                                encoder.write_u8(Message::MessageSync.to_u8());
                                let yjs_message = decoder.read_u8().unwrap();
                                match YrsMessage::from_u8(yjs_message) {
                                    YrsMessage::MessageYjsSyncStep1 => {
                                        let message = decoder.read_buf().unwrap();
                                        encoder.write_u8(YrsMessage::MessageYjsSyncStep2.to_u8());
                                        let update = doc.transact_mut().encode_state_as_update_v1(
                                            &StateVector::decode_v1(message).unwrap(),
                                        );
                                        encoder.write_buf(update);
                                        let message = encoder.to_vec();
                                        // println!("Sending MessageYjsSyncStep2 {:?}", message);
                                        socket.send(ws::Message::Binary(message)).await.unwrap();
                                    }
                                    YrsMessage::MessageYjsSyncStep2 => {
                                        let message = decoder.read_buf().unwrap();
                                        let update = Update::decode_v1(&message).unwrap();
                                        let _ = doc.transact_mut().apply_update(update);
                                        // println!("Received MessageYjsSyncStep2 {:?}", message);
                                    }
                                    YrsMessage::MessageYjsUpdate => {
                                        let message = decoder.read_buf().unwrap();
                                        let update = Update::decode_v1(&message).unwrap();
                                        let _ = doc.transact_mut().apply_update(update);
                                        // println!("Received MessageYjsUpdate {:?}", message);
                                    }
                                    _ => {}
                                }
                            }
                            Message::MessageAwareness => {}
                            _ => {}
                        }
                    }
                    Some(ws::Message::Text(_)) => {}
                    Some(ws::Message::Close(_)) => {
                        // println!("`{}` disconnected.", room);
                        break;
                    },
                    Some(ws::Message::Ping(_)) => {
                        // println!("Received PING");
                        socket.send(ws::Message::Pong(vec![])).await.unwrap();
                    }
                    Some(ws::Message::Pong(_)) => {
                        // println!("Received PONG");
                    },
                    None => {}
                }
            },
            ping = rx.recv() => {
                match ping {
                    Some(_)=>{
                        // println!("Sending awareness message: PING");
                        socket.send(ws::Message::Ping(vec![])).await.unwrap();
                    },
                    None=>{}
                }
            },
        }
    }
    ping_spawn.abort();
    // println!("`{}` disconnected.", who);
}
