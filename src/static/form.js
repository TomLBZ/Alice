// static/form.js
let editingService = null;

function showForm(mode, data = {}) {
  const modal = document.getElementById("modal");
  modal.classList.remove("hidden");
  const title = document.getElementById("modalTitle");
  title.textContent = mode === "edit" ? "Edit Service" : "Create Service";

  const form = document.getElementById("serviceForm");
  form.reset();

  if (mode === "edit") {
    form.name.value = data.name;
    form.version.value = data.version;
    form.description.value = data.description;
    form.mode.value = data.mode;
    editingService = data;
    form.name.disabled = true;
    form.version.disabled = true;
  } else {
    editingService = null;
    form.name.disabled = false;
    form.version.disabled = false;
  }
}

function cancelEdit() {
  editingService = null;
  document.getElementById("modal").classList.add("hidden");
}

async function submitForm(e) {
  e.preventDefault();
  const form = e.target;
  const formData = new FormData(form);

  let endpoint = "/service/create";
  if (editingService) {
    formData.append("new_name", form.name.value);
    formData.append("new_version", form.version.value);
    endpoint = "/service/update";
  }

  const res = await fetch(endpoint, { method: "POST", body: formData });
  if (res.ok) {
    cancelEdit();
    loadServices();
  } else {
    alert("Error submitting form");
  }
}
