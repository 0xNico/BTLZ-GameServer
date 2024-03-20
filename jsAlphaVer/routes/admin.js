const express = require('express');
const router = express.Router();
const fs = require('fs');
const path = require('path');

// Utility functions
function readJsonFile(filename) {
  const filePath = path.join(__dirname, `../models/${filename}.json`);
  return JSON.parse(fs.readFileSync(filePath, 'utf-8'));
}

function writeJsonFile(filename, data) {
  const filePath = path.join(__dirname, `../models/${filename}.json`);
  fs.writeFileSync(filePath, JSON.stringify(data, null, 2), 'utf-8');
}

function validateUrl(url) {
  try {
    new URL(url);
    return true;
  } catch (_) {
    return false;
  }
}

function validateMoveSet(moveSet) {
  if (!Array.isArray(moveSet) || moveSet.length === 0) return false;
  return moveSet.every(move => typeof move.name === 'string' && 
    Array.isArray(move.damage_range) && move.damage_range.length === 2 &&
    typeof move.cooldown === 'number');
}

function validateBattleEntry(entry) {
  return typeof entry.name === 'string' &&
    typeof entry.tier === 'number' &&
    Array.isArray(entry.hp_range) && entry.hp_range.length === 2 &&
    Array.isArray(entry.xp_range) && entry.xp_range.length === 2 &&
    validateMoveSet(entry.move_set);
}

function validateClassEntry(entry) {
  return typeof entry.name === 'string' &&
    validateUrl(entry.armor_url) &&
    typeof entry.hp_boost === 'number' &&
    typeof entry.dodge_chance === 'number' &&
    typeof entry.description === 'string' &&
    validateMoveSet(entry.move_set);
}

function validateWeaponEntry(entry) {
  return typeof entry.name === 'string' &&
    typeof entry.rarity === 'string' &&
    typeof entry.description === 'string' &&
    validateUrl(entry.weapon_url) &&
    typeof entry.dmg_boost === 'number' &&
    Array.isArray(entry.dmg_range) && entry.dmg_range.length === 2;
}

// Add new entry to a category
function addNewEntry(category, newEntry) {
  const data = readJsonFile(`${category}.json`);

  let isValid = false;
  switch (category) {
    case 'battles':
      isValid = validateBattleEntry(newEntry);
      break;
    case 'classes':
      isValid = validateClassEntry(newEntry);
      break;
    case 'weapons':
      isValid = validateWeaponEntry(newEntry);
      break;
  }

  if (!isValid) {
    throw new Error(`Invalid entry structure for ${category}.`);
  }

  data.push(newEntry);
  writeJsonFile(`${category}.json`, data);
}

// Routes
router.get('/:category', (req, res) => {
  const { category } = req.params;
  try {
    const data = readJsonFile(`${category}`);
    res.json(data);
  } catch (error) {
    res.status(404).json({ message: `Category ${category} not found.` });
  }
});

router.post('/:category', (req, res) => {
  const { category } = req.params;
  const newEntry = req.body;

  try {
    addNewEntry(category, newEntry);
    res.status(201).json({ message: 'Entry added successfully.', newEntry });
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
});

module.exports = router;
