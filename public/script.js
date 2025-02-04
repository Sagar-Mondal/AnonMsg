document.addEventListener("DOMContentLoaded", function () {
  const submitButton = document.getElementById("submit");
  const messageBox = document.getElementById("message");
  const loadingOverlay = document.getElementById("loading-overlay");

  submitButton.addEventListener("click", function () {
    const message = messageBox.value.trim();
    if (!message) {
      Swal.fire("Error", "Message cannot be empty!", "error");
      return;
    }

    submitButton.disabled = true;
    loadingOverlay.classList.remove("hidden");

    fetch("/post-message", {
      method: "POST",
      body: message,
    })
      .then((response) => response.text())
.then(data => {
            Swal.fire({ title: "Success", icon: "success", text: data.message || "Message sent successfully!",  confirmButtonColor: '#A87849' });
        })
        .catch(error => {
            Swal.fire({ title: "Error", icon: "warning", text: error.message || "Something went wrong!", confirmButtonColor: '#A87849' });
        })
      .finally(() => {
        submitButton.disabled = false;
        loadingOverlay.classList.add("hidden");
        messageBox.value = "";
      });
  });
});
