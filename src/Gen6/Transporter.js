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

const formatSettings = state => {
  const settings = {
    min_ivs: [
      parseInt(state.minIVs.hp),
      parseInt(state.minIVs.atk),
      parseInt(state.minIVs.def),
      parseInt(state.minIVs.spa),
      parseInt(state.minIVs.spd),
      parseInt(state.minIVs.spe),
    ],
    max_ivs: [
      parseInt(state.maxIVs.hp),
      parseInt(state.maxIVs.atk),
      parseInt(state.maxIVs.def),
      parseInt(state.maxIVs.spa),
      parseInt(state.maxIVs.spd),
      parseInt(state.maxIVs.spe),
    ],
    rng_state: parseInt(state.rng_state, 16),
    min_advances: state.min_advances,
    max_advances: state.max_advances,
    delay: state.delay,
    iv_rolls: state.iv_rolls,
    is_shiny: state.is_shiny,
    tid: parseInt(state.tid, 10),
  };
  return settings;
};

export function Gen6Transporter() {
  const { t } = useTranslation();
  const [searching, setSearching] = React.useState(false);

  const [state, setState] = React.useState({
    rng_state: '',
    min_advances: 0,
    max_advances: 10000,
    delay: 0,
    minIVs: { hp: 0, atk: 0, def: 0, spa: 0, spd: 0, spe: 0 },
    maxIVs: { hp: 31, atk: 31, def: 31, spa: 31, spd: 31, spe: 31 },
    iv_rolls: false,
    is_shiny: false,
    tid: 0,
    target: {
      is_set: false,
      advances: 0,
      psv: 0,
      ivs: [0, 0, 0, 0, 0, 0],
      hidden_power: 'Fighting',
      pid: 0,
    },
  });

  const [results, setResults] = React.useState([
    {
      advances: 0,
      psv: 0,
      pid: 0,
      ivs: [0, 0, 0, 0, 0, 0],
      hidden_power: 'Fighting',
    },
  ]);

  const settings = formatSettings(state);

  const handleSubmit = event => {
    event.preventDefault();

    setSearching(true);

    return calculatePokemon(settings).then(data => {
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
      <Results setState={setState} results={results} state={state} />
    </Box>
  );
}
