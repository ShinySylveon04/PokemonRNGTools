import React from 'react';
import { calculate_pokemon } from '../wasm/Cargo.toml';
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
  const [state, setState] = React.useState({
    state0: BigInt('0xe1e16bc81e378a0b'),
    state1: BigInt('0xa79a405a9d7f5849'),
    tid: 32125,
    sid: 998,
    shiny: 0,
    encounter: 0,
    shiny_charm: false,
  });
  const [results, setResults] = React.useState({
    advances: 0,
    shiny_value: 0,
    state0: BigInt('0xe1e16bc81e378a0b'),
    state1: BigInt('0xa79a405a9d7f5849'),
    ec: 0,
    pid: 0,
  });
  const { state0, state1, tid, sid, shiny, encounter, shiny_charm } = state;
  const test = event => {
    event.preventDefault();
    const shiny_result = calculate_pokemon(
      state0,
      state1,
      tid,
      sid,
      shiny,
      encounter,
      shiny_charm,
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
        <TrainerInfo setState={setState} state={state} />
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
