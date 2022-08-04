import React from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import CircularProgress from '@mui/material/CircularProgress';
import Toolbar from '@mui/material/Toolbar';

import { wrap } from 'comlink';
import { useTranslation } from 'react-i18next';

import { RNGInfo } from './RNGInfo';
import { Filters } from './Filters';
import { Results } from './Results';

const calculatePokemon = wrap(new Worker('./workers/getResults.js'));

export function Underground() {
  const { t } = useTranslation();
  const [searching, setSearching] = React.useState(false);
  const [state, setState] = React.useState({
    state0: '',
    state1: '',
    state2: '',
    state3: '',
    shiny: 4,
    min: 0,
    max: 10000,
    delay: 0,
    nature: [25],
    ability: 3,
    species: 0,
    genderRatio: 256,
    gender: 256,
    diglett_boost: false,
    minIVs: { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 },
    maxIVs: { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 },
    version: 2,
    room: 2,
    story_flag: 6,
  });

  const [results, setResults] = React.useState([
    {
      advances: 0,
      shiny_value: 'None',
      state0: 0,
      state1: 0,
      ec: 0,
      pid: 0,
      encounter: 0,
      nature: 'Any',
      ability: 0,
      ivs: [0, 0, 0, 0, 0, 0],
      gender: 256,
    },
  ]);

  const {
    state0,
    state1,
    state2,
    state3,
    shiny,
    min,
    max,
    delay,
    nature,
    ability,
    species,
    gender,
    diglett_boost,
    minIVs,
    maxIVs,
    version,
    room,
    story_flag
  } = state;

  const handleSubmit = event => {
    event.preventDefault();
    setSearching(true);

    return calculatePokemon(
      parseInt(state0, 16),
      parseInt(state1, 16),
      parseInt(state2, 16),
      parseInt(state3, 16),
      shiny,
      min,
      max,
      delay,
      nature,
      ability,
      species,
      gender,
      diglett_boost,
      [
        parseInt(minIVs.hp),
        parseInt(minIVs.atk),
        parseInt(minIVs.def),
        parseInt(minIVs.spa),
        parseInt(minIVs.spd),
        parseInt(minIVs.spe),
      ],
      [
        parseInt(maxIVs.hp),
        parseInt(maxIVs.atk),
        parseInt(maxIVs.def),
        parseInt(maxIVs.spa),
        parseInt(maxIVs.spd),
        parseInt(maxIVs.spe),
      ],
      version,
      room,
      story_flag
    ).then(data => {
      setResults(data), setSearching(false);
    });
  };

  return (
    <Box
      component="form"
      autoComplete="off"
      onSubmit={handleSubmit}
      sx={{
        width: { sm: '75%' },
        maxWidth: '1000px',
        ml: 'auto',
        mr: 'auto',
        mb: '30px',
        display: 'flex',
        flexDirection: 'column',
      }}
    >
      <Toolbar />
      <RNGInfo setState={setState} state={state} />
      <Filters setState={setState} state={state} />
      <Button
        disabled={searching}
        type="submit"
        variant="contained"
        fullWidth
        sx={{ margin: '10px', ml: 'auto', mr: 'auto', maxWidth: '300px' }}
      >
        {searching ? <CircularProgress size={24} /> : t('Search')}
      </Button>
      <Results results={results} state={state} />
    </Box>
  );
}
