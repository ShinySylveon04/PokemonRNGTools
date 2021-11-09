import React from 'react';
import { calculate_shiny } from '../wasm/Cargo.toml';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Container from '@mui/material/Container';

import { TrainerInfo } from './TrainerInfo';
import { RNGInfo } from './RNGInfo';
import { Filters } from './Filters';
import { Results } from './Results';

export function App() {
  // 32435 + 51677
  // 0x7EB3C9DD
  const [state, setState] = React.useState({state0: 0, state1: 0, tsv: 0});
  const [shiny, setShiny] = React.useState(0);
  const [results, setResults] = React.useState({
    advances: 0,
    shiny: 0,
    state0: 0,
    state1: 0,
  });
  const { state0, state1, tsv } = state;
  const test = event => {
    event.preventDefault();
    const shiny_result = calculate_shiny(state0, state1, tsv, shiny);
    setResults(shiny_result);
  };

  console.log(state);
  console.log(results);
  return (
    <Container>
      <Box
        component="form"
        autoComplete="off"
        onSubmit={test}
        sx={{
          width: { sm: '75%' },
          maxWidth: '800px',
          ml: 'auto',
          mr: 'auto',
          mb: '20px',
          display: 'flex',
          flexDirection: 'column',
        }}
      >
        <Typography variant="h3" gutterBottom align="center">
          Sword & Shield RNG
        </Typography>
        <TrainerInfo setState={setState}/>
        <RNGInfo setState={setState}/>
        <Filters setShiny={setShiny} shiny={shiny}/>
        <Button
          type="submit"
          variant="contained"
          fullWidth
          sx={{ margin: '10px', ml: 'auto', mr: 'auto', maxWidth: '300px'}}
        >
          Search
        </Button>
        <Results results={results}/>
      </Box>
    </Container>
  );
}
