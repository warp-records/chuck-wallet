use coin::block::*;
use coin::user::*;
use hex;
use std::{iter::Once, sync::Mutex};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

static STATE: Mutex<Option<State>> = Mutex::new(None);
static USER: Mutex<Option<State>> = Mutex::new(None);

fn main() {
    console_error_panic_hook::set_once();

    let mut state = State::with_genesis_block();
    let mut first = &mut state.blocks[0];

    //alert("Hello world!");
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");

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

#[wasm_bindgen]
pub fn get_balance() -> u64 {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let input = document
        .get_element_by_id("privkey")
        .expect("should find input element");

    let input_element = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();

    let priv_key = input_element.value();
    let pub_key = User::from_priv(priv_key.as_str()).verifying.into();
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();

    state.get_balance(pub_key)
}

#[wasm_bindgen]
pub fn spend(amt: u64) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let input = document
        .get_element_by_id("privkey")
        .expect("should find input element");

    let input_element = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();

    let priv_key = input_element.value();
    let signing = User::from_priv(priv_key.as_str()).signing;

    let rand_user = User::random();
    state.blocks[0].transact(&mut state.utxo_set, &signing, &rand_user.verifying, amt);
}

/*
#[wasm_bindgen]
pub fn spend(amt: u64) {
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let input = document
        .get_element_by_id("wallet_addr")
        .expect("should find input element");

    let input_element = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();

    let priv_key = input_element.value();
    let verifying = User::from_priv(priv_key.as_str()).verifying.into();

    let rand_user = User::random();
    state.blocks[0].transact(&mut state.utxo_set, &signing, &rand_user.verifying, amt);
}
 */
