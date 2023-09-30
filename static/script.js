const audioContext = new (window.AudioContext || window.webkitAudioContext)();
let mediaStream;
let audioInput;
let websocket;
let isStreaming = false;

const startStreamingButton = document.getElementById('startStreaming');
const stopStreamingButton = document.getElementById('stopStreaming');
const sendToServerButton = document.getElementById('sendToServer');

startStreamingButton.addEventListener('click', startStreaming);
stopStreamingButton.addEventListener('click', stopStreaming);
sendToServerButton.addEventListener('click', sendToServer);

function startStreaming() {
    navigator.mediaDevices.getUserMedia({ audio: true })
        .then(function (stream) {
            mediaStream = stream;
            audioInput = audioContext.createMediaStreamSource(stream);

            // Connect audio input to a WebSocket for streaming
            websocket = new WebSocket('ws://your-server-url'); // Replace with your server URL
            websocket.addEventListener('open', function (event) {
                isStreaming = true;
                startStreamingButton.disabled = true;
                stopStreamingButton.disabled = false;
                sendToServerButton.disabled = true;
            });

            // Send audio data to the WebSocket
            audioInput.connect(audioContext.destination);
            audioInput.onaudioprocess = function (e) {
                if (isStreaming) {
                    const audioData = e.inputBuffer.getChannelData(0); // Get audio data from the microphone
                    websocket.send(audioData.buffer); // Send audio data to the server
                }
            };
        })
        .catch(function (error) {
            console.error('Error accessing the microphone:', error);
        });
}

function stopStreaming() {
    isStreaming = false;
    if (mediaStream) {
        mediaStream.getTracks().forEach(track => track.stop());
    }
    if (websocket) {
        websocket.close();
    }
    startStreamingButton.disabled = false;
    stopStreamingButton.disabled = true;
    sendToServerButton.disabled = false;
}

function sendToServer() {
    // Implement sending audio data to the server here
    // You'll need to handle this part based on your server implementation.
    // The WebSocket connection is already established.
}