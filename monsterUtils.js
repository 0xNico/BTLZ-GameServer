// Assuming monsterData is loaded as shown in your server printout
const fs = require('fs');
const path = require('path');

// Load monster data from JSON file
const data = JSON.parse(fs.readFileSync(path.join(__dirname, 'models', 'battles.json'), 'utf-8'));
const monsterData = data.battles; // Adjust to use the 'battles' key

// Rest of your monsterUtils.js code...

function selectMonster() {
    if (!monsterData || monsterData.length === 0) {
        console.error("No monster data available.");
        return null;
    }

    const monsterIndex = Math.floor(Math.random() * monsterData.length);
    const selectedMonster = {...monsterData[monsterIndex]};

    if (!selectedMonster || !selectedMonster.hp_range || !selectedMonster.xp_range) {
        console.error("Selected monster does not have valid hp_range or xp_range", selectedMonster);
        return null; // Handle this scenario appropriately
    }

    selectedMonster.currentHP = getRandomInRange(selectedMonster.hp_range[0], selectedMonster.hp_range[1]);
    selectedMonster.givenXP = getRandomInRange(selectedMonster.xp_range[0], selectedMonster.xp_range[1]);

    return selectedMonster;
}

function getRandomInRange(min, max) {
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

module.exports = {
    selectMonster,
};
