/* Status codes */
const STATUS_OK = 200;
const STATUS_BAD_REQUEST = 400;
const STATUS_UNPROCESSABLE_CONTENT = 422;
const STATUS_INTERNAL_SERVER_ERROR = 500;

const form = document.querySelector("form");
const deleteBtn = document.querySelector("button[type='button']");

form.addEventListener("submit", async (ev) => {
  ev.preventDefault();

  let data = Object.fromEntries(new FormData(form));
  data.code = parseInt(data.code);

  console.log("Updating link entry with:", data);

  let response = await fetch("/api/links", {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });

  console.log(`PATCH /api/links: ${response.status} (${response.statusText})`);

  if (response.status == STATUS_OK) {
    // Everything went well
    console.log("Link updation successful");
    alert(`Link updation successful.`);
  } else if (response.status == STATUS_BAD_REQUEST) {
    // Validation error
    let body = await response.json();

    console.error(body.err);
    alert(`Invalid inputs: ${body.err}`);
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

deleteBtn.addEventListener("click", async (ev) => {
  if (!form.reportValidity()) return;

  let data = Object.fromEntries(new FormData(form));
  data.code = parseInt(data.code);
  delete data.href;

  console.log("Deleting link entry with:", data);

  let response = await fetch("/api/links", {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });

  console.log(`DELETE /api/links: ${response.status} (${response.statusText})`);

  if (response.status == STATUS_OK) {
    // Everything went well
    console.log("Link deletion successful");
    alert(`Link deletion successful.`);
  } else if (response.status == STATUS_BAD_REQUEST) {
    // Validation error
    let body = await response.json();

    console.error(body.err);
    alert(`Invalid inputs: ${body.err}`);
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
