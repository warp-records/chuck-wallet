import init, { get_nonce } from "./dist/chuck-wallet.js";

async function run() {
  alert("loading");
  await init(); // Initialize the WASM module
  alert("done init");
  const nonce = get_nonce(); // Call the Rust function
  console.log("Nonce:", nonce);
  alert("mined");
}

run();
