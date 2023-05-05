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

const calculateTID = wrap<AnyPromiseFunction>(
  new Worker('./workers/getResults.ts'),
);

const formatSettings = state => {
  const settings = {
    rng_state: [state.state0, state.state1, state.state2, state.state3].map(
      num => parseInt(num, 16),
    ),
    min_advances: state.min_advances,
    max_advances: state.max_advances,
    id: state.id
      .split('\n')
      .filter(id => id.length !== 0)
      .map(id => parseInt(id, 10)),
    filter_type: state.id_filter,
  };
  return settings;
};

export function TID() {
  const { t } = useTranslation();

  const [searching, setSearching] = React.useState(false);
  const [state, setState] = React.useState({
    state0: '',
    state1: '',
    state2: '',
    state3: '',
    min_advances: 0,
    max_advances: 10000,
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

  const settings = formatSettings(state);

  const handleSubmit = event => {
    event.preventDefault();

    setSearching(true);

    return calculateTID(settings).then(data => {
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
