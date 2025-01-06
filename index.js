/*
//THIS IS NECESSARY
 */

window.addEventListener("TrunkApplicationStarted", (event) => {
  const wasm = window.wasmBindings;
  //document.getElementById("nonce").innerText = "0x" + nonce.toString(16);
  //alert("hello");

  var prevInput = "";
  document.getElementById("privkey").addEventListener("blur", (event) => {
    const input = event.target.value;
    if (input == prevInput || input == "") {
      return;
    }
    prevInput = input;
    wasm.set_user(input);

    var bal = wasm.get_balance();
    document.getElementById("balance").textContent = "Balance: " + bal;
    document.getElementById("wallet_addr").textContent = wasm.get_pub_key();
  });

  document.getElementById("sendbtn").addEventListener("click", (event) => {
    var amt = document.getElementById("sendamt").value;
    var key = document.getElementById("privkey").value;
    if (amt > 0) {
      wasm.spend(key, amt);
      var bal = wasm.get_balance(key);
      document.getElementById("balance").textContent = "Balance: " + bal;
    }
  });
});
