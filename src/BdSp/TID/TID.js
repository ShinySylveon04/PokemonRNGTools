import React from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import CircularProgress from '@mui/material/CircularProgress';
import Toolbar from '@mui/material/Toolbar';

import { wrap } from 'comlink';

import { RNGInfo } from './RNGInfo';
import { Filters } from './Filters';
import { Results } from './Results';

const calculateTID = wrap(new Worker('./workers/getResults.js'));

export function TID() {
  const [searching, setSearching] = React.useState(false);
  const [state, setState] = React.useState({
    state0: 0x12345678,
    state1: 0x12345678,
    state2: 0x12345678,
    state3: 0x12345678,
    min: 0,
    max: 10000,
    id: '',
    id_filter: 'None',
  });

  const [results, setResults] = React.useState([
    {
      advances: 0,
      state0: 0,
      state1: 0,
      g8tid: 0,
      tid: 0,
      sid: 0,
      tsv: 0,
    },
  ]);

  const { state0, state1, state2, state3, min, max, id, id_filter } = state;

  const handleSubmit = event => {
    event.preventDefault();

    setSearching(true);

    return calculateTID(
      state0,
      state1,
      state2,
      state3,
      min,
      max,
      id.split('\n'),
      id_filter,
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
        {searching ? <CircularProgress size={24} /> : 'Search'}
      </Button>
      <Results results={results} state={state} />
    </Box>
  );
}
