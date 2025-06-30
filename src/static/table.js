// static/table.js
let currentSelected = null;

async function loadServices() {
  const res = await fetch("/services");
  const data = await res.json();
  const tbody = document.getElementById("servicesBody");
  const testSelector = document.getElementById("testSelector");
  const currentVal = testSelector.value;

  tbody.innerHTML = "";
  testSelector.innerHTML = "";
  let found = false;

  Object.entries(data).forEach(([id, svc]) => {
    const tr = document.createElement("tr");
    tr.className = id === currentSelected ? "active" : "";

    const link = document.createElement("a");
    link.href = "#";
    link.textContent = svc.name;
    link.onclick = () => {
      navigator.clipboard.writeText(`${location.origin}/ws/serve/${svc.name}/${svc.version}`);
    };
    const nameTd = document.createElement("td");
    nameTd.appendChild(link);
    tr.appendChild(nameTd);

    tr.innerHTML += `<td>${svc.mode}</td><td>${svc.description}</td><td>${svc.sessions}</td><td>${svc.uptime}s</td>`;
    const actions = document.createElement("td");
    actions.innerHTML = `<button onclick='showForm("edit", ${JSON.stringify(svc)})'>Edit</button>
                         <button onclick='deleteService("${svc.name}", "${svc.version}")'>Delete</button>`;
    tr.appendChild(actions);

    tbody.appendChild(tr);

    const opt = document.createElement("option");
    opt.value = id;
    opt.text = id;
    testSelector.appendChild(opt);
    if (id === currentVal) {
      opt.selected = true;
      currentSelected = id;
      found = true;
    }
  });

  if (!found) currentSelected = null;
}

async function deleteService(name, version) {
  const form = new FormData();
  form.append("name", name);
  form.append("version", version);
  await fetch("/service/delete", { method: "POST", body: form });
  loadServices();
}
