import React from 'react';

import TextField from '@mui/material/TextField';
import Grid from '@mui/material/Grid';

import { useTranslation } from 'react-i18next';

import { setRngStateFromClipboard } from '../Utils/setRngState';

export const RNGStates = ({ state, setState }) => {
  const { t } = useTranslation();

  const handlePaste = event => setRngStateFromClipboard(event, setState);
  return (
    <React.Fragment>
      <Grid container item sm={6} xs={12} justifyContent="center">
        <TextField
          onPaste={event => handlePaste(event)}
          fullWidth
          autoComplete="off"
          inputProps={{
            inputMode: 'text',
            maxLength: 8,
          }}
          value={state.state0}
          id="seed0"
          label={t('Seed 0')}
          variant="outlined"
          onChange={event => {
            setState(state => ({
              ...state,
              state0: event.target.value,
            }));
          }}
        />
      </Grid>
      <Grid container item sm={6} xs={12} justifyContent="center">
        <TextField
          autoComplete="off"
          fullWidth
          inputProps={{
            inputMode: 'text',
            maxLength: 8,
          }}
          value={state.state1}
          id="seed1"
          label={t('Seed 1')}
          variant="outlined"
          onChange={event => {
            setState(state => ({
              ...state,
              state1: event.target.value,
            }));
          }}
        />
      </Grid>
      <Grid container item sm={6} xs={12} justifyContent="center">
        <TextField
          autoComplete="off"
          fullWidth
          inputProps={{
            inputMode: 'text',
            maxLength: 8,
          }}
          value={state.state2}
          id="seed2"
          label={t('Seed 2')}
          variant="outlined"
          onChange={event => {
            setState(state => ({
              ...state,
              state2: event.target.value,
            }));
          }}
        />
      </Grid>
      <Grid container item sm={6} xs={12} justifyContent="center">
        <TextField
          autoComplete="off"
          fullWidth
          inputProps={{
            inputMode: 'text',
            maxLength: 8,
          }}
          value={state.state3}
          id="seed3"
          label={t('Seed 3')}
          variant="outlined"
          onChange={event => {
            setState(state => ({
              ...state,
              state3: event.target.value,
            }));
          }}
        />
      </Grid>
    </React.Fragment>
  );
};
