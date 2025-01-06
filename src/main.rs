use coin::block::*;
use coin::user::*;
use hex;
use std::os;
use std::{iter::Once, sync::Mutex};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

static STATE: Mutex<Option<State>> = Mutex::new(None);
static USER: Mutex<Option<User>> = Mutex::new(None);

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
    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();

    let nonce = state.blocks[0].mine(); // Mine the nonce
    state.blocks[0].nonce = nonce; // Update the nonce in the state

    nonce // Return the mined nonce
}

#[wasm_bindgen]
pub fn set_user(priv_key: &str) -> Result<(), JsValue> {
    let mut user = User::try_from_priv(priv_key).map_err(|_| JsValue::null())?;

    let mut user_guard = USER.lock().unwrap();
    *user_guard = Some(user);

    Ok(())
}

#[wasm_bindgen]
pub fn get_balance() -> Result<u64, JsValue> {

    let mut user = USER.lock().unwrap();
    let user = user.as_ref().ok_or(JsValue::null())?;
    let pub_key = user.verifying.into();

    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();

    Ok(state.get_balance(pub_key))
}

#[wasm_bindgen]
pub fn get_pub_key() -> Result<String, JsValue> {
    let user = USER.lock().unwrap();
    let user = user.as_ref().ok_or(JsValue::null())?;
    Ok(hex::encode_upper(user.verifying.to_encoded_point(false)))
}

#[wasm_bindgen]
pub fn spend(priv_key: &str, amt: u64) -> Result<(), JsValue> {

    let signing = User::try_from_priv(priv_key)
        .map_err(|_| JsValue::null())?
        .signing;

    let mut state = STATE.lock().unwrap();
    let state = state.as_mut().unwrap();

    let rand_user = User::random();
    state.blocks[0].transact(&mut state.utxo_set, &signing, &rand_user.verifying, amt);

    Ok(())
}

//#[wasm_bindgen]
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
