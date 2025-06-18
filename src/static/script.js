let ws = null;
let currentServiceId = "";

function refreshServices() {
  fetch("/services")
    .then(res => res.json())
    .then(data => {
      const tbody = document.getElementById("servicesBody");
      const selector = document.getElementById("testSelector");
      tbody.innerHTML = "";
      selector.innerHTML = "<option value=''>Select service...</option>";
      Object.entries(data).forEach(([id, svc]) => {
        const row = document.createElement("tr");
        row.innerHTML = `
          <td>${id}</td>
          <td>${svc.description}</td>
          <td>${svc.sessions}</td>
          <td>${svc.uptime}</td>
          <td>
            <button onclick="editService('${svc.name}', '${svc.version}', \`${svc.description}\`)">Edit</button>
            <button onclick="deleteService('${svc.name}', '${svc.version}')">Delete</button>
          </td>
        `;
        tbody.appendChild(row);
        const opt = document.createElement("option");
        opt.value = `${svc.name}:${svc.version}`;
        opt.textContent = `${svc.name}:${svc.version}`;
        selector.appendChild(opt);
      });
    });
}

function editService(name, version, desc) {
  const form = document.getElementById("editForm");
  form.name.value = name;
  form.version.value = version;
  form.description.value = desc;
  form.new_name.value = "";
  form.new_version.value = "";
  form.exec_file.value = "";
  document.getElementById("editSection").style.display = "block";
}

function cancelEdit() {
  document.getElementById("editSection").style.display = "none";
}

document.getElementById("editForm").onsubmit = async (e) => {
  e.preventDefault();
  const form = new FormData(e.target);
  await fetch("/service/update", { method: "POST", body: form });
  cancelEdit();
  refreshServices();
};

async function deleteService(name, version) {
  const form = new FormData();
  form.append("name", name);
  form.append("version", version);
  await fetch("/service/delete", { method: "POST", body: form });
  refreshServices();
}

document.getElementById("createForm").onsubmit = async (e) => {
  e.preventDefault();
  const form = new FormData(e.target);
  await fetch("/service/create", { method: "POST", body: form });
  e.target.reset();
  refreshServices();
};

function selectTestService() {
  const selected = document.getElementById("testSelector").value;
  document.getElementById("testBox").style.display = selected ? "block" : "none";
  currentServiceId = selected;
}

function startSession() {
  if (ws) ws.close();
  const [name, version] = currentServiceId.split(":");
  ws = new WebSocket(`ws://${location.host}/ws/serve/${name}/${version}`);
  ws.onmessage = (e) => {
    document.getElementById("testOutput").value = e.data;
  };
}

function sendInput() {
  const input = document.getElementById("testInput").value;
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(input);
  }
}

function endSession() {
  if (ws) ws.close();
  ws = null;
}

refreshServices();
setInterval(refreshServices, 3000);
