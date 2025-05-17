const logoutBtnEl = document.getElementById("logout-btn");
const stayBtnEl = document.getElementById("stay-btn");

logoutBtnEl.addEventListener("click", async () => {
  await fetch("/v1/auth/logout", {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
    },
  });
  window.location.replace("/");
});

stayBtnEl.addEventListener("click", () => {
  window.location.replace("/my/profile");
});
