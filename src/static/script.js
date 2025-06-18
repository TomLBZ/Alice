let ws = null;
let currentServiceId = "";
let currentMode = "function";

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
          <td>${svc.mode || "function"}</td>
          <td>${svc.description}</td>
          <td>${svc.sessions}</td>
          <td>${svc.uptime}</td>
          <td>
            <button onclick="editService('${svc.name}', '${svc.version}', \`${svc.description}\`, '${svc.mode}')">Edit</button>
            <button onclick="deleteService('${svc.name}', '${svc.version}')">Delete</button>
          </td>
        `;
        tbody.appendChild(row);

        const opt = document.createElement("option");
        opt.value = `${svc.name}:${svc.version}`;
        opt.textContent = `${svc.name}:${svc.version}`;
        opt.dataset.mode = svc.mode || "function";
        selector.appendChild(opt);
      });
    });
}

function editService(name, version, desc, mode) {
  const form = document.getElementById("editForm");
  form.name.value = name;
  form.version.value = version;
  form.description.value = desc;
  form.mode.value = mode;
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
  const selector = document.getElementById("testSelector");
  const selected = selector.value;
  document.getElementById("testBox").style.display = selected ? "block" : "none";
  currentServiceId = selected;
  const selectedOption = selector.selectedOptions[0];
  currentMode = selectedOption.dataset.mode || "function";
  document.getElementById("testOutput").value = "";  // Clear output on new select
}

function startSession() {
  if (ws) ws.close();
  const [name, version] = currentServiceId.split(":");
  ws = new WebSocket(`ws://${location.host}/ws/serve/${name}/${version}`);

  ws.onmessage = (e) => {
    const box = document.getElementById("testOutput");
    if (currentMode === "function") {
      box.value = e.data;
    } else {
      box.value += e.data;
      box.scrollTop = box.scrollHeight;
    }
  };
}

function sendInput() {
  const input = document.getElementById("testInput").value;
  if (ws && ws.readyState === WebSocket.OPEN) {
    if (currentMode === "function") {
      document.getElementById("testOutput").value = "";  // clear before each call
    }
    ws.send(input);
  }
}

function endSession() {
  if (ws) ws.close();
  ws = null;
}

refreshServices();
setInterval(refreshServices, 3000);
