/*
//THIS IS NECESSARY
 */

window.addEventListener("TrunkApplicationStarted", async (event) => {
  const wasm = window.wasmBindings;
  //document.getElementById("nonce").innerText = "0x" + nonce.toString(16);
  //alert("hello");

  //document.getElementById("loading").innerHTML = "&#x2705";
  try {
    const res = await wasm.fetch_blockchain();
    document.getElementById("loading").innerHTML = "Loaded blockchain &#x2705";
    document.getElementById("content").style = "display: block";
    setTimeout(() => {
      document.getElementById("loading").style.transition = "opacity 1s";
      document.getElementById("loading").style.opacity = "0";
      //setTimeout(() => {
     //   document.getElementById("loading").remove();
     // }, 1000); // Remove after 1 second (duration of the fade-out)
    }, 2000); // Start fade-out after 1 second
  } catch (error) {
    document.getElementById("loading").innerHTML = "Failed to load blockchain &#10060;";
  }

  document.getElementById("privkey").addEventListener("blur", (event) => {
    const input = event.target.value;
    if (input == "") {
      return;
    }

    try {
      wasm.set_user(input);
    } catch (error) {
      document.getElementById("login_msg").innerHTML = "Invalid private key &#10060;";
      document.getElementById("privkey").value = "";
      return;
    }
    document.getElementById("login_msg").innerHTML = "Logged in &#x2705";
    document.getElementById("privkey").value = "";

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
