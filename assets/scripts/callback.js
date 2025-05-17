const infoTextEl = document.querySelector(".info-text");

async function exec() {
  const { access_token, token_type, expires_in } = Object.fromEntries(
    new URLSearchParams(window.location.hash.slice(1))
  );
  if (!access_token || !token_type || !expires_in) {
    infoTextEl.textContent = "Invalid token response :c";
    console.error("Invalid token response");
    return setTimeout(() => {
      window.location.replace("/");
    }, 1500);
  }

  try {
    const res = await fetch("/v1/auth/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        access_token,
        token_type,
        expires_in,
      }),
    });
    console.log(res);
    if (res.status !== 200) {
      const data = await res.json();
      console.log(data);
      throw new Error(data.error);
    }

    infoTextEl.textContent = "Successfully logged in! Redirecting...";
    setTimeout(() => {
      window.location.replace("/my/profile");
    }, 1500);
  } catch (err) {
    console.error(err);
    infoTextEl.textContent = `Error while logging in :c\n${err.message}`;
    setTimeout(() => {
      window.location.replace("/");
    }, 1500);
  }
}

(async () => {
  await exec();
})();
