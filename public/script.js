document.addEventListener("DOMContentLoaded", function () {
  const submitButton = document.getElementById("submit");
  const messageBox = document.getElementById("message");
  const loadingOverlay = document.getElementById("loading-overlay");

  submitButton.addEventListener("click", function () {
    const message = messageBox.value.trim();
    if (!message) {
      Swal.fire({ title: "Error", icon: "error", text: "Message cannot be empty!", confirmButtonColor: '#A87849' });
      return;
    }

    submitButton.disabled = true;
    loadingOverlay.classList.remove("hidden");

    fetch("/post-message", {
      method: "POST",
      body: message,
    })
      .then((response) => {
        if (!response.ok) {
          return response.text().then(text => { throw new Error(text || `Error ${response.status}`); });
        }
        return response.text();
      })
      .then(data => {
        Swal.fire({ title: "Success", icon: "success", text: "Message sent successfully!", confirmButtonColor: '#A87849' });
      })
      .catch(error => {
        Swal.fire({ title: "Error", icon: "warning", text: "Something went wrong!", confirmButtonColor: '#A87849' });
      })
      .finally(() => {
        submitButton.disabled = false;
        loadingOverlay.classList.add("hidden");
        messageBox.value = "";
      });
  });
});
