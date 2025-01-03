use coin::block::*;
use hex;
use std::{iter::Once, sync::Mutex};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn main() {
    console_error_panic_hook::set_once();

    let mut state = State::with_genesis_block();
    let mut first = &mut state.blocks[0];

    //alert("Hello world!");
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_element("p").unwrap();
    text_node.set_id("nonce");
    text_node.set_text_content(Some("Mining genesis block..."));

    body.append_child(text_node.as_ref())
        .expect("Failed to append text");

    //for _ in 0..10 { };
    //text_node.set_data(&format!("First block nonce: {}", hex::encode(first.nonce.to_be_bytes())));

    *STATE.lock().unwrap() = Some(state);
}

#[wasm_bindgen]
pub fn get_nonce() -> u64 {
    let mut state = STATE.lock().unwrap(); // Lock the Mutex to access the state
    let state = state.as_mut().unwrap(); // Get a mutable reference to the state

    let nonce = state.blocks[0].mine(); // Mine the nonce
    state.blocks[0].nonce = nonce; // Update the nonce in the state

    nonce // Return the mined nonce
}
