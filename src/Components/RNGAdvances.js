import React from 'react';

import TextField from '@mui/material/TextField';
import Grid from '@mui/material/Grid';

import { useTranslation } from 'react-i18next';

export const RNGAdvances = ({ state, setState }) => {
  const { t } = useTranslation();

  return (
    <React.Fragment>
      <Grid item sm={6} md={3} xs={12}>
        <TextField
          inputProps={{
            inputMode: 'numeric',
            pattern: '[0-9]*',
          }}
          fullWidth
          id="min"
          label={t('Min Advances')}
          variant="outlined"
          value={state.min_advances}
          onChange={event =>
            setState(state => ({
              ...state,
              min_advances:
                event.target.value.length === 0
                  ? ''
                  : parseInt(event.target.value),
            }))
          }
        />
      </Grid>
      <Grid item sm={6} md={3} xs={12}>
        <TextField
          inputProps={{
            inputMode: 'numeric',
            pattern: '[0-9]*',
          }}
          fullWidth
          id="max"
          label={t('Max Advances')}
          variant="outlined"
          value={state.max_advances}
          onChange={event =>
            setState(state => ({
              ...state,
              max_advances:
                event.target.value.length === 0
                  ? ''
                  : parseInt(event.target.value),
            }))
          }
        />
      </Grid>
    </React.Fragment>
  );
};
