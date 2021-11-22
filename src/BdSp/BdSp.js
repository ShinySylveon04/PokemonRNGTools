import React from 'react';
import { calculate_pokemon_bdsp } from '../../wasm/Cargo.toml';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Container from '@mui/material/Container';

import { RNGInfo } from './RNGInfo';
import { Filters } from './Filters';
import { Results } from './Results';

export function BdSp() {
  const [state, setState] = React.useState({
    state0: 0x3ad6c1f0,
    state1: 0x0ccea7a7,
    state2: 0x3aa81fb1,
    state3: 0xf423082b,
    shiny: false,
    min: 0,
    max: 1000000,
    nature: 25,
    ability: 2,
  });

  const [results, setResults] = React.useState([
    {
      advances: 0,
      shiny_value: 0,
      state0: 0,
      state1: 0,
      ec: 0,
      pid: 0,
      nature: 0,
      ability: 0,
    },
  ]);

  const { state0, state1, state2, state3, shiny, min, max } = state;

  const handleSubmit = event => {
    event.preventDefault();
    const shiny_result = calculate_pokemon_bdsp(
      state0,
      state1,
      state2,
      state3,
      shiny,
      min,
      max,
    );
    setResults(shiny_result);
  };

  console.log(state);
  console.log(results);

  return (
    <Container>
      <Box
        component="form"
        autoComplete="off"
        onSubmit={handleSubmit}
        sx={{
          width: { sm: '75%' },
          maxWidth: '800px',
          ml: 'auto',
          mr: 'auto',
          mb: '30px',
          display: 'flex',
          flexDirection: 'column',
        }}
      >
        <Typography variant="h3" gutterBottom align="center">
          Brilliant Diamond & Shining Pearl RNG
        </Typography>
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
        <Results results={results} />
      </Box>
    </Container>
  );
}
