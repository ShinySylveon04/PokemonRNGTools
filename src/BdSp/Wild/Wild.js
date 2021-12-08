import React from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Container from '@mui/material/Container';
import CircularProgress from '@mui/material/CircularProgress';

import { wrap } from 'comlink';

import { RNGInfo } from './RNGInfo';
import { Filters } from './Filters';
import { Results } from './Results';

const calculatePokemon = wrap(new Worker('./workers/getResults.js'));

export function Wild() {
  const [searching, setSearching] = React.useState(false);
  const [state, setState] = React.useState({
    state0: 0x41e3a1cb,
    state1: 0x39f7a401,
    state2: 0x32bcc45e,
    state3: 0x564639f7,
    shiny: 4,
    min: 0,
    max: 10000,
    delay: 1,
    nature: [25],
    ability: 3,
    encounter: [12],
    genderRatio: 256,
    gender: 256,
    minIVs: { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 },
    maxIVs: { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 },
    lead: 0,
  });

  const [results, setResults] = React.useState([
    {
      advances: 0,
      shiny_value: 0,
      state0: 0,
      state1: 0,
      ec: 0,
      pid: 0,
      encounter: 0,
      nature: 0,
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
    encounter,
    genderRatio,
    gender,
    minIVs,
    maxIVs,
    lead,
  } = state;

  const handleSubmit = event => {
    event.preventDefault();

    setSearching(true);

    return calculatePokemon(
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
      encounter,
      genderRatio,
      gender,
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
      lead,
    ).then(data => {
      setResults(data), setSearching(false);
    });
  };

  return (
    <Container>
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
        <RNGInfo setState={setState} state={state} />
        <Filters setState={setState} state={state} />
        <Button
          disabled={searching}
          type="submit"
          variant="contained"
          fullWidth
          sx={{ margin: '10px', ml: 'auto', mr: 'auto', maxWidth: '300px' }}
        >
          {searching ? <CircularProgress size={24} /> : 'Search'}
        </Button>
        <Results results={results} state={state} />
      </Box>
    </Container>
  );
}
