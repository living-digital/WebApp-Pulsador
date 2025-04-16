
```markdown
# Door Control Frontend in Yew

## Overview
This is a simple web application built with [Yew](https://yew.rs/), a Rust framework for creating web apps with WebAssembly. The application extracts a token from the URL's query string and uses it to send a POST request to a Node‑RED endpoint. Depending on the response (e.g., "Door opened", "Token incorrect", or "Token not provided"), the app displays a notification with the server’s message.

## Features
- **Token Extraction:** The app reads a token from the URL’s query string (e.g., `/?token=CLIENT_TOKEN`).
- **API Request:** When the "Open" button is clicked, a POST request is sent to `http://devel.livingdigitalsolutions.com/admin/pulsador` with the token included in the query string.
- **Asynchronous Processing:** Uses `gloo_net::http::Request` and `wasm_bindgen_futures` to send the request asynchronously.
- **User Feedback:** Displays the response received from the server (or any network errors) directly on the web page.

## Requirements
- Rust and its WebAssembly toolchain (e.g., via [wasm-pack](https://rustwasm.github.io/wasm-pack/))
- A web server to serve the generated WebAssembly (e.g., `basic-http-server` or any static file server)
- Node‑RED setup to process the POST request and return the appropriate response

## How It Works
1. **Token Extraction:**  
   The application attempts to extract a token from the URL query string. For instance, if the URL is:
   ```
   http://devel.livingdigitalsolutions.com/?token=CLIENT_TOKEN_UNIQUE
   ```
   then the token `"CLIENT_TOKEN_UNIQUE"` is saved.

2. **Sending the Request:**  
   When the user clicks the "Open" button, the application forms a URL including the token as a query parameter:
   ```
   http://devel.livingdigitalsolutions.com/admin/pulsador?token=CLIENT_TOKEN_UNIQUE
   ```
   It then sends a POST request to that endpoint.

3. **Handling the Response:**  
   The server (Node‑RED) processes the token. It returns a text message such as "Door opened" if the token is valid or an error message if it isn’t. The application updates the UI to display this message in a notification element.

4. **Error Handling:**  
   If the POST request fails (for example, due to a network error or CORS issues), the error message is captured and displayed on the page.

## Code Structure
- **create:**  
  Extracts the token from the current URL's query string.
  
- **update:**  
  - On `Msg::OpenDoor`, it sends an asynchronous POST request to the Node‑RED endpoint with the token.
  - On `Msg::ResponseReceived`, it updates the feedback message to display in the UI.

- **view:**  
  Renders a basic UI with a button to trigger the door open action and displays a notification with the feedback message.

## How to Build and Run
1. **Build the project with `wasm-pack`:**
   ```bash
   wasm-pack build --target web
   ```
2. **Serve the generated files:**  
   Use your preferred static file server. For example, you can use `basic-http-server`:
   ```bash
   basic-http-server ./pkg
   ```
3. **Open your application in a browser:**  
   Make sure to include a token in the URL, e.g.:
   ```
   http://localhost:8080/?token=hacker
   ```

## Dependencies
- [Yew](https://yew.rs/)
- [gloo_net](https://github.com/rustwasm/gloo)
- [wasm_bindgen_futures](https://rustwasm.github.io/wasm-bindgen/reference/futures.html)
- [web_sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)

