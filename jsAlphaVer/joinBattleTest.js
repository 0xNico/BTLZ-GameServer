const io = require('socket.io-client');

// Adjust the URL to match your game server's address and port
const serverUrl = 'http://localhost:3000';
const socket = io.connect(serverUrl);

socket.on('connect', () => {
    console.log('Connected to the game server.');

    // Join a specific battle instance, replace 'battle1' with your battle identifier
    const battleId = 'battle1';
    socket.emit('joinBattle', battleId);
    console.log(`Requested to join battle: ${battleId}`);
});

// Listening for updates from the server about the battle
socket.on('battleUpdate', (data) => {
    console.log('Battle Update:', data);

    // If you want to automatically disconnect after receiving the update
    // socket.disconnect();
});

socket.on('disconnect', () => {
    console.log('Disconnected from the game server.');
});
