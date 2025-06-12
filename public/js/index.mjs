const form = document.querySelector("form");

form.addEventListener("submit", async (ev) => {
  ev.preventDefault();

  let data = Object.fromEntries(new FormData(form));

  console.log("Sending POST /api/links/create with: ", data);

  let response = await fetch("/api/links/create", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });

  let body = await response.json();

  if (response.status != 200) {
    console.error(
      "POST /api/links/create returned",
      response.status,
      response.statusText,
      body
    );
  } else {
    console.log("POST /api/links/create returned 200 (OK).");
  }
});
