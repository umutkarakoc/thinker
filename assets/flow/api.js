let api = async (action, params) => {
  let result = await fetch(`/api/${action}`, {
    method: "post",
    body: JSON.stringify(params),
    headers: {
      "content-type": "application/json"
    }
  });

  result = await result.json();

  return result;
}

let post = async (action, params) => {
  let result = await fetch(`/api/${action}`, {
    method: "POST",
    body: JSON.stringify(params),
    headers: {
      "content-type": "application/json"
    }
  });

  result = await result.json();

  return result;
}

let patch = async (action, params) => {
  let result = await fetch(`/api/${action}`, {
    method: "PATCH",
    body: JSON.stringify(params),
    headers: {
      "content-type": "application/json"
    }
  });

  result = await result.json();

  return result;
}

let get = async (action, params) => {
  let result = await fetch(`/api/${action}`, {
    method: "GET",
    body: JSON.stringify(params)
  });

  result = await result.json();

  return result;
}