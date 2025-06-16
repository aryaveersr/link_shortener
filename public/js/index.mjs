/* Status codes */
const STATUS_OK = 200;
const STATUS_BAD_REQUEST = 400;
const STATUS_CONFLICT = 409;
const STATUS_UNPROCESSABLE_CONTENT = 422;
const STATUS_INTERNAL_SERVER_ERROR = 500;

const form = document.querySelector("form");

form.addEventListener("submit", async (ev) => {
  ev.preventDefault();

  let data = Object.fromEntries(new FormData(form));

  console.log("Creating a link entry with:", data);

  let response = await fetch("/api/links", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });

  console.log(`POST /api/links: ${response.status} (${response.statusText})`);

  if (response.status == STATUS_OK) {
    // Everything went well
    let body = await response.json();

    console.log("Link creation successful");
    alert(
      `Link creation successful. Your edit code is ${body.code}. Keep it safe.`
    );
  } else if (response.status == STATUS_BAD_REQUEST) {
    // Validation error
    let body = await response.json();

    console.error(body.err);
    alert(`Invalid inputs: ${body.err}`);
  } else if (response.status == STATUS_CONFLICT) {
    // Slug already exists
    alert("This slug already exists");
  } else if (response.status == STATUS_UNPROCESSABLE_CONTENT) {
    // Request wasn't well formed
    alert("Your request couldn't be processed. Sorry :(");
  } else if (response.status == STATUS_INTERNAL_SERVER_ERROR) {
    // Server error
    alert("Your request couldn't be processed. Sorry :(");
  } else {
    // Unknown error
    let body = await response.json();

    console.error(body);
    alert("Your request couldn't be processed. Sorry :(");
  }
});
