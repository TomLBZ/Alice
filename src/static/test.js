// static/test.js
let ws = null;

function selectTestService() {
    if (ws) return; // Prevent changing service while a session is active
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
    const inputBox = document.getElementById("testInput");
    output.value = "";
    inputBox.value = "";
    inputBox.disabled = true;

    ws.onmessage = (e) => {
        output.value += e.data;
        output.scrollTop = output.scrollHeight;
    };

    ws.onopen = () => {
        document.getElementById("startBtn").disabled = true;
        document.getElementById("sendBtn").disabled = false;
        document.getElementById("endBtn").disabled = false;
        const selector = document.getElementById("testSelector");
        selector.title = "End the current session to change service";
        selector.disabled = true;
        inputBox.disabled = false;
    };

    ws.onclose = () => {
        ws = null;
        document.getElementById("startBtn").disabled = false;
        document.getElementById("sendBtn").disabled = true;
        document.getElementById("endBtn").disabled = true;
        const selector = document.getElementById("testSelector");
        selector.title = "Select a service to start a session";
        selector.disabled = false;
        inputBox.disabled = true;
    };
}

function sendInput() {
    const input = document.getElementById("testInput").value;
    if (ws) ws.send(input);
}

function endSession() {
    if (ws) ws.close();
}
