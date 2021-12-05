import React from 'react';
import { calculate_pokemon_bdsp_underground } from '../../../wasm/Cargo.toml';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Container from '@mui/material/Container';

import { RNGInfo } from './RNGInfo';
import { Filters } from './Filters';
import { Results } from './Results';

export function Underground() {
  const [state, setState] = React.useState({
    state0: 0,
    state1: 0,
    state2: 0,
    state3: 0,
    shiny: true,
    min: 0,
    max: 10000,
    delay: 0,
    nature: [25],
    ability: 3,
    encounter: 12,
    genderRatio: 256,
    gender: 256,
    tiles: 0,
    large_room: false,
    diglett_boost: false,
  });

  const [results, setResults] = React.useState([
    {
      advances: 0,
      is_shiny: 0,
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
    tiles,
    large_room,
    diglett_boost,
  } = state;

  const handleSubmit = event => {
    event.preventDefault();
    const shiny_result = calculate_pokemon_bdsp_underground(
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
      tiles,
      large_room,
      diglett_boost,
    );
    setResults(shiny_result);
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
          type="submit"
          variant="contained"
          fullWidth
          sx={{ margin: '10px', ml: 'auto', mr: 'auto', maxWidth: '300px' }}
        >
          Search
        </Button>
        <Results results={results} state={state} />
      </Box>
    </Container>
  );
}
