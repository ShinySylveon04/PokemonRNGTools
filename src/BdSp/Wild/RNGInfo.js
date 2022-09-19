import React from 'react';
import Typography from '@mui/material/Typography';
import TextField from '@mui/material/TextField';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';

import { useTranslation } from 'react-i18next';

import { LeadFilter } from '../../Components/LeadFilter';
import { RNGStates } from '../../Components/RNGStates';

export const RNGInfo = ({ setState, state }) => {
  const { t } = useTranslation();

  return (
    <Paper variant="outlined" sx={{ padding: '10px', m: '10px' }}>
      <Grid
        container
        spacing={2}
        direction="row"
        justifyContent="center"
        alignItems="center"
      >
        <Grid item xs={12}>
          <Typography variant="h6" align="left" color="primary">
            {t('RNG Info')}
          </Typography>
        </Grid>
        <RNGStates state={state} setState={setState} />
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
        <Grid item sm={6} md={3} xs={12}>
          <LeadFilter state={state} setState={setState} />
        </Grid>
      </Grid>
    </Paper>
  );
};
