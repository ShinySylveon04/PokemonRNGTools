const fs = require('fs');
const ugPokemon = require('./src/Resources/UgPokemonData.json');
const species = require('./species');

const pokeRates = ugPokemon.table.reduce((accumulator, pokemon) => {
  const speciesName = species[pokemon.monsno] ?? 'UNKNOWN';
  const isEncounterable = pokemon.flagrate.findIndex(rate => rate > 0) != -1;

  if (isEncounterable) {
    accumulator[speciesName] = {
      species: pokemon.monsno,
      flagRate: pokemon.flagrate,
    };
  }

  return accumulator;
}, {});

const code = `export const POKEMON = ${JSON.stringify(pokeRates, null, 2)};`;

fs.writeFileSync('./pokeRates.js', code, { encoding: 'utf8' });
