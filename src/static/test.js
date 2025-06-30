// static/test.js
let ws = null;

function selectTestService() {
  const selector = document.getElementById("testSelector");
  currentSelected = selector.value;
  loadServices();
}

function startSession() {
  if (ws) return;
  const id = document.getElementById("testSelector").value;
  const [name, version] = id.split(":");
  ws = new WebSocket(`${location.origin.replace("http", "ws")}/ws/serve/${name}/${version}`);

  const output = document.getElementById("testOutput");
  output.value = "";

  ws.onmessage = (e) => {
    output.value += e.data + "\n";
    output.scrollTop = output.scrollHeight;
  };

  ws.onopen = () => {
    document.getElementById("startBtn").disabled = true;
    document.getElementById("sendBtn").disabled = false;
    document.getElementById("endBtn").disabled = false;
  };

  ws.onclose = () => {
    ws = null;
    document.getElementById("startBtn").disabled = false;
    document.getElementById("sendBtn").disabled = true;
    document.getElementById("endBtn").disabled = true;
  };
}

function sendInput() {
  const input = document.getElementById("testInput").value;
  if (ws) ws.send(input);
}

function endSession() {
  if (ws) ws.close();
}
