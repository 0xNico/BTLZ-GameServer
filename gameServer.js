const express = require('express');
const http = require('http');
const socketIo = require('socket.io');
const adminRoutes = require('./routes/admin');
const monsterUtils = require('./monsterUtils')

const app = express();
const server = http.createServer(app); // Create an HTTP server
const io = socketIo(server); // Attach Socket.IO to the HTTP server

const port = process.env.PORT || 3000;

app.use(express.json());
app.use('/admin', adminRoutes);

//Utility Functions
function formatMoveSet(moveSet) {
    return moveSet.map(move => {
        let moveDetails = `${move.name}, Cooldown: ${move.cooldown}s`;

        if (move.damage_range) {
            moveDetails += `, Damage Range: ${move.damage_range.join(' to ')}`;
        }

        if (move.heal_range) {
            moveDetails += `, Heal Range: ${move.heal_range.join(' to ')}`;
        }

        return moveDetails;
    }).join('; ');
}

// Socket.IO connection handler
io.on('connection', (socket) => {
    console.log(`New client connected: ${socket.id}`);

    // Inside the 'joinBattle' event listener
    socket.on('joinBattle', (battleId) => {
        console.log(`Client ${socket.id} joined battle ${battleId}`);
        socket.join(battleId);

        // Generate a monster for the battle
        const monster = monsterUtils.selectMonster();

        // Update the monster object to include the formatted move set directly
        const monsterWithFormattedMoves = {
            ...monster,
            move_set: formatMoveSet(monster.move_set) // Format and include in the monster object
        };

        // Now, the formatted move set is part of the monster's details
        console.log(`Monster selected for battle ${battleId}:`, monsterWithFormattedMoves);

        // Emitting the monster details with formatted move set to the battle room
        io.to(battleId).emit('battleUpdate', { monster: monsterWithFormattedMoves });
        });

    
    socket.on('disconnect', () => {
        console.log(`Client disconnected: ${socket.id}`);
    });
});

// Modify app.listen to server.listen to include our Socket.IO server
server.listen(port, () => {
    console.log(`BTLZ SERVER - Game server is running on http://localhost:${port}`);
});
