import React from 'react';

import TextField from '@mui/material/TextField';
import Grid from '@mui/material/Grid';

import { useTranslation } from 'react-i18next';

export const RNGDelay = ({ state, setState }) => {
  const { t } = useTranslation();

  return (
    <Grid item sm={6} md={3} xs={12}>
      <TextField
        inputProps={{
          inputMode: 'numeric',
          pattern: '[0-9]*',
        }}
        fullWidth
        id="delay"
        label={t('Delay')}
        variant="outlined"
        value={state.delay}
        onChange={event =>
          setState(state => ({
            ...state,
            delay:
              event.target.value.length === 0
                ? ''
                : parseInt(event.target.value),
          }))
        }
      />
    </Grid>
  );
};
