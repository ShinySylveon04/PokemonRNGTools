import React from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import CircularProgress from '@mui/material/CircularProgress';
import Toolbar from '@mui/material/Toolbar';

import { wrap } from 'comlink';
import { useTranslation } from 'react-i18next';

// const calculatePokemon = wrap(new Worker('./workers/getResults.js'));

export function RS() {
  const { t } = useTranslation();
  const [searching, setSearching] = React.useState(false);

  return (
    <Box
      component="form"
      autoComplete="off"
      // onSubmit={handleSubmit}
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
      {/* <RNGInfo setState={setState} state={state} />
      <Filters setState={setState} state={state} /> */}
      <Button
        disabled={searching}
        type="submit"
        variant="contained"
        fullWidth
        sx={{ margin: '10px', ml: 'auto', mr: 'auto', maxWidth: '300px' }}
      >
        {searching ? <CircularProgress size={24} /> : t('Search')}
      </Button>
      {/* <Results results={results} state={state} /> */}
    </Box>
  );
}
