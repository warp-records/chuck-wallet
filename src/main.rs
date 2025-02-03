use coin::block::*;
use coin::user::*;
use hex;
use std::net::TcpStream;
use std::os;
use std::{iter::Once, sync::Mutex};
use web_sys::window;
use std::collections::{HashMap};
use lazy_static::lazy_static;
use coin::frametype::*;
//use tungstenite::{http::{Method, Request}, client::*};
//use tokio_tungstenite_wasm::{connect_async, tungstenite, tungstenite::protocol::Message};
use serde::*;
use futures::{SinkExt, stream::StreamExt};
use web_sys::WebSocket;

use ws_stream_wasm::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State {
        blocks: Vec::new(),
        old_utxo_set: HashMap::new(),
        utxo_set: HashMap::new(),
    });
}

static USER: Mutex<Option<User>> = Mutex::new(None);
static WS_STREAM: Mutex<Option<WsStream>> = Mutex::new(None);


fn main() {
    console_error_panic_hook::set_once();

    //let mut state = State::with_genesis_block();
    //let mut first = &mut state.blocks[0];

    //alert("Hello world!");
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");

    //for _ in 0..10 { };
    //text_node.set_data(&format!("First block nonce: {}", hex::encode(first.nonce.to_be_bytes())));

    /*
    wasm_bindgen_futures::spawn_local(async move {
        let res = fetch_blockchain().await;
        assert!(res.is_ok());
        let mut state = STATE.lock().unwrap();
        assert!(state.verify_all_and_update().is_ok());

        alert("Verified server blockchain");
    }); */
}

#[wasm_bindgen]
pub fn set_user(priv_key: &str) -> Result<(), JsError> {
    let mut user = User::try_from_priv(priv_key).map_err(|_| JsError::new(""))?;

    let mut user_guard = USER.lock().unwrap();
    *user_guard = Some(user);

    Ok(())
}

#[wasm_bindgen]
pub fn get_balance() -> Result<u64, JsError> {

    let mut user = USER.lock().unwrap();
    let user = user.as_ref().ok_or(JsError::new(""))?;
    let pub_key = user.verifying.into();

    let mut state = STATE.lock().unwrap();

    Ok(state.get_balance(pub_key))
}

#[wasm_bindgen]
pub fn get_pub_key() -> Result<String, JsError> {
    let user = USER.lock().unwrap();
    let user = user.as_ref().ok_or(JsError::new(""))?;
    Ok(hex::encode_upper(user.verifying.to_encoded_point(false)))
}

#[wasm_bindgen]
pub async fn spend(pub_key: &str, amt: u64) -> Result<(), JsError> {

    let spender = USER.lock().unwrap();
    let spender = spender.as_ref().unwrap();//ok_or(JsError::new("No user set"))?;
    let spender_priv = &spender.signing;

    let recipient_pub = try_public_from_str(pub_key)
        .map_err(|_| JsError::new("Invalid pub key"))?;

    let mut state = STATE.lock().unwrap();

    let mut last_block = state.blocks.last().unwrap().clone();
    let new_tx = last_block.transact(&mut state.utxo_set, &spender_priv, &recipient_pub.into(), amt)
        .map_err(|_| JsError::new("Invalid transaction"))?;


    //upload to server
    let mut guard = WS_STREAM.lock().unwrap();

    let mut ws_stream = match &mut *guard {
        Some(stream) => stream,
        None => {
            let (_, stream) = WsMeta::connect(&format!("ws://{SERVER_IP}:{PORT}"), None)
                .await
                .map_err(|_| JsError::new("Failed to connect to WebSocket"))?;
            *guard = Some(stream);
            guard.as_mut().unwrap()
        }
    };


    let serialized = bincode::serialize(&ClientFrame::TxFrame(vec![new_tx.clone()])).unwrap();
    ws_stream.send(WsMessage::Binary(serialized))
        .await
        .map_err(|_| JsError::new("Failed to send transaction to server"))?;


    Ok(())
}


//Because error handling in javascript is almost as catastrophic
//as Trump's first term
//here are the error strings this may return:

//"InvalidFrame",
//"InvalidBlockchain"


#[wasm_bindgen]
pub async fn fetch_blockchain() -> Result<(), JsError> {

    // Create a WebSocket connection using ws_stream_wasm
    let (_ws_meta, mut ws_stream) = WsMeta::connect(&format!("ws://{SERVER_IP}:{PORT}"), None)
        .await
        .map_err(|_| JsError::new("CantConnect"))?;

    // Serialize the ClientFrame using bincode
    let frame = ClientFrame::GetBlockchain;
    let serialized = bincode::serialize(&frame)
        .map_err(|_| JsError::new("Failed to serialize frame"))?;

    // Send the serialized frame over WebSocket
    ws_stream
        .send(WsMessage::Binary(serialized))
        .await
        .map_err(|_| JsError::new("Failed to send message"))?;

    // Wait for a response
    let msg = ws_stream.next().await.unwrap();

    // Deserialize the response into a ServerFrame
    let blockchain = match msg {
        WsMessage::Binary(data) => {
            if let Ok(ServerFrame::BlockChain(blockchain)) = bincode::deserialize(&data) {
                blockchain
            } else {
                return Err(JsError::new("InvalidFrame"));
            }
        }
        _ => return Err(JsError::new("InvalidFrame")),
    };

    // Update the blockchain in the state
    let mut state = STATE.lock().unwrap();
    state.blocks = blockchain;

    state.verify_all_and_update().map_err(|_| JsError::new("InvalidBlockchain"))?;

    Ok(())
}
