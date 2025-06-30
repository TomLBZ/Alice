// static/script.js
document.addEventListener("DOMContentLoaded", () => {
  loadServices();
  setInterval(loadServices, 3000);

  document.getElementById("createBtn").onclick = () => showForm("create");
  document.getElementById("modalClose").onclick = cancelEdit;
  document.getElementById("cancelEdit").onclick = cancelEdit;

  document.getElementById("serviceForm").onsubmit = submitForm;
});
