const axios = require('axios');

const serverUrl = 'http://localhost:3000/admin'; // Adjust the port if your server runs on a different one

async function getCategory(category) {
    try {
        const response = await axios.get(`${serverUrl}/${category}`);
        const data = response.data;
        if (Array.isArray(data)) {
            data.forEach((item, index) => formatAndPrintItem(item, `Entry ${index + 1}`));
        } else {
            formatAndPrintItem(data, 'Data');
        }
    } catch (error) {
        console.error(`Error fetching category ${category}:`, error.response?.data || error.message);
    }
}

function sanitizeOutput(output) {
    return output.replace(/["{}\[\]]/g, '');
}

function formatAndPrintItem(item, title) {
    console.log('\n' + title);
    Object.keys(item).forEach(key => {
        let value = item[key];
        if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
            value = JSON.stringify(value, null, 2); // Convert objects to string for sanitization
        }
        if (Array.isArray(value) && key !== 'move_set') { // Special handling for arrays that are not move_set
            console.log(`${capitalizeFirstLetter(key)}:`);
            value.forEach(subItem => console.log(`  - ${typeof subItem === 'object' ? sanitizeOutput(JSON.stringify(subItem, null, 2)) : subItem}`));
        } else if (key === 'move_set') { // Directly handle move_set printing
            console.log(`${capitalizeFirstLetter(key)}:`);
            value.forEach(move => {
                console.log(`  Move: ${move.name}`);
                if (move.damage_range) {
                    console.log(`    Damage Range: ${move.damage_range.join(' to ')}`);
                }
                if (move.heal_range) {
                    console.log(`    Heal Range: ${move.heal_range.join(' to ')}`);
                }
                console.log(`    Cooldown: ${move.cooldown}`);
            });
        } else { // Directly print the value, sanitize if it's a string
            console.log(`${capitalizeFirstLetter(key)}: ${typeof value === 'string' ? sanitizeOutput(value) : value}`);
        }
    });
}

function capitalizeFirstLetter(string) {
    return string.charAt(0).toUpperCase() + string.slice(1);
}

async function addEntry(category, entry) {
    try {
        const response = await axios.post(`${serverUrl}/${category}`, entry);
        console.log(response.data);
    } catch (error) {
        console.error(`Error adding entry to ${category}:`, error.response?.data || error.message);
    }
}

// Examples of using the functions
// Ensure server is running before executing these commands
// You can uncomment these lines one at a time to test different functionalities

getCategory('battles');
getCategory('classes');
getCategory('weapons');

// addEntry('battles', {
//   name: "DoomKnight",
//   tier: 4,
//   hp_range: [1500, 5000],
//   xp_range: [50, 200],
//   move_set: [
//       {name: "Flame Slash", damage_range: [10, 25], cooldown: 3},
//       {name: "Kinetic Slice", damage_range: [15, 33], cooldown: 9}
//   ]
// });

// addEntry('classes', {
//   name: "Beserker",
//   armor_url: "https://example.com/armor/beserker.webp",
//   hp_boost: 1,
//   dodge_chance: 0.08,
//   description: "Beserkers are fast striking fearless warriors who care not for healing. DPS gud.",
//   move_set: [
//       {name: "Enraged Flurry", damage_range: [35, 55], cooldown: 1.5},
//       {name: "Unleash Rage", damage_range: [80, 120], cooldown: 6}
//   ]
// });

// addEntry('weapons', {
//   name: "Frostbane",
//   rarity: "Mythic",
//   description: "A Mythical Frost-Breathing Greatsword infused with The Eternal Cold.",
//   weapon_url: "https://example.com/weapon/frostbane.webp",
//   dmg_boost: 1.1,
//   dmg_range: [100, 200]
// });
