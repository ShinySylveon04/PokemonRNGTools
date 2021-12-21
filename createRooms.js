const fs = require('fs');
const room = require('./src/Resources/UgEncounters/SpaciousCave.json');
const species = require('./species');

const ROOM_NUMBER = 2;

const VERSION = {
  BOTH: 3,
  DIAMOND: 1,
  PEARL: 2,
};

const diamond = [];
const pearl = [];

const pokemon = room.table;
for (let i = 0; i < pokemon.length; i++) {
  const singleEncounter = pokemon[i];

  if (singleEncounter.version === VERSION.BOTH) {
    diamond.push(singleEncounter);
    pearl.push(singleEncounter);
  }

  if (singleEncounter.version === VERSION.DIAMOND) {
    diamond.push(singleEncounter);
  }

  if (singleEncounter.version === VERSION.PEARL) {
    pearl.push(singleEncounter);
  }
}

const formatPokemon = pokemon => {
  return `{ zukanflag: ${pokemon.zukanflag}, ...POKEMON.${
    species[pokemon.monsno] ?? 'UNKNOWN'
  }},`;
};

const code = `
import { POKEMON } from './pokeRates';

export const ROOM_${ROOM_NUMBER} = {
    DIAMOND: [
        ${diamond.map(formatPokemon).join('\n')}
    ],
    PEARL: [
        ${pearl.map(formatPokemon).join('\n')}
    ]
};
`;

fs.writeFileSync(`./SpaciousCave.js`, code, { encoding: 'utf8' });
