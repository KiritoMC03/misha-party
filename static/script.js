// const socket = new WebSocket("ws://localhost:3030/ws");
//
// socket.onopen = () => {
//     console.log("WebSocket connection established.");
//     // Send data or perform actions when the connection is open.
// };
//
// socket.onmessage = (event) => {
//     console.log(`Received message: ${event.data}`);
//     // Handle incoming messages from the server.
// };
//
// socket.onclose = (event) => {
//     if (event.wasClean) {
//         console.log(`WebSocket closed cleanly, code=${event.code}, reason=${event.reason}`);
//     } else {
//         console.error(`WebSocket connection died`);
//     }
// };
//
// socket.onerror = (error) => {
//     console.error(`WebSocket error: ${error.message}`);
// };
//
// // Send data to the server
// function sendMessage() {
//     const message = "Hello, WebSocket Server!";
//     socket.send(message);
// }