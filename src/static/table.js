// static/table.js
let currentSelected = null;

function buildServiceRow(svc, id) {
    const tr = document.createElement("tr");
    if (id === currentSelected) tr.classList.add("active");

    // Service name (copyable link)
    const nameTd = document.createElement("td");
    const link = document.createElement("a");
    link.href = "#";
    link.textContent = svc.name;
    link.title = "Click to copy WebSocket URL";
    link.onclick = (e) => {
        e.preventDefault();
        const wsUrl = `${location.origin.replace("http", "ws")}/ws/serve/${svc.name}/${svc.version}`;
        navigator.clipboard.writeText(wsUrl).then(() => {
            const originalTitle = link.title;
            link.title = "Copied!";
            setTimeout(() => (link.title = originalTitle), 1000);
        });
    };
    nameTd.appendChild(link);
    tr.appendChild(nameTd);

    // Mode
    const modeTd = document.createElement("td");
    modeTd.textContent = svc.mode;
    tr.appendChild(modeTd);

    // Description
    const descTd = document.createElement("td");
    descTd.textContent = svc.description;
    tr.appendChild(descTd);

    // Sessions
    const sessTd = document.createElement("td");
    sessTd.textContent = svc.sessions;
    tr.appendChild(sessTd);

    // Uptime
    const upTd = document.createElement("td");
    upTd.textContent = `${svc.uptime}s`;
    tr.appendChild(upTd);

    // Actions
    const actions = document.createElement("td");
    actions.innerHTML = `
        <button onclick='showForm("edit", ${JSON.stringify(svc)})'>Edit</button>
        <button onclick='deleteService("${svc.name}", "${svc.version}")'>Delete</button>
    `;
    tr.appendChild(actions);

    return tr;
}

async function loadServices() {
    const res = await fetch("/services");
    if (!res.ok) return;

    const data = await res.json();
    const tbody = document.getElementById("servicesBody");
    tbody.innerHTML = "";

    const selector = document.getElementById("testSelector");
    const selectedValue = selector.value;
    selector.innerHTML = "";

    Object.entries(data).forEach(([id, svc]) => {
        const option = document.createElement("option");
        option.value = id;
        option.textContent = `${svc.name}:${svc.version}`;
        selector.appendChild(option);

        const tr = buildServiceRow(svc, id);
        tbody.appendChild(tr);
    });

    // Restore previous selection if still present
    if ([...selector.options].some(o => o.value === selectedValue)) {
        selector.value = selectedValue;
    } else {
        currentSelected = selector.value;
    }
}

async function deleteService(name, version) {
    const form = new FormData();
    form.append("name", name);
    form.append("version", version);
    await fetch("/service/delete", { method: "POST", body: form });
    loadServices();
}
