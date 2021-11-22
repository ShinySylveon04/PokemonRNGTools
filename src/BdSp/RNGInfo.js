import React from 'react';
import Typography from '@mui/material/Typography';
import TextField from '@mui/material/TextField';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';

export const RNGInfo = ({ setState, state }) => {
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
            RNG Info
          </Typography>
        </Grid>
        <Grid container item sm={6} xs={12} justifyContent="center">
          <TextField
            autoComplete="new-password"
            fullWidth
            inputProps={{
              inputMode: 'text',
              maxLength: 8,
            }}
            id="state0"
            label="State 0"
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                state0: parseInt(event.target.value, 16),
              }));
            }}
          />
        </Grid>
        <Grid container item sm={6} xs={12} justifyContent="center">
          <TextField
            autoComplete="new-password"
            fullWidth
            inputProps={{
              inputMode: 'text',
              maxLength: 8,
            }}
            id="state1"
            label="State 1"
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                state1: parseInt(event.target.value, 16),
              }));
            }}
          />
        </Grid>
        <Grid container item sm={6} xs={12} justifyContent="center">
          <TextField
            autoComplete="new-password"
            fullWidth
            inputProps={{
              inputMode: 'text',
              maxLength: 8,
            }}
            id="state2"
            label="State 2"
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                state2: parseInt(event.target.value, 16),
              }));
            }}
          />
        </Grid>
        <Grid container item sm={6} xs={12} justifyContent="center">
          <TextField
            autoComplete="new-password"
            fullWidth
            inputProps={{
              inputMode: 'text',
              maxLength: 8,
            }}
            id="state3"
            label="State 3"
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                state3: parseInt(event.target.value, 16),
              }));
            }}
          />
        </Grid>

        <Grid item sm={6} md={3} xs={12}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="min"
            label="Min Advances"
            variant="outlined"
            value={state.min}
            onChange={event =>
              setState(state => ({
                ...state,
                min: parseInt(event.target.value),
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
            label="Max Advances"
            variant="outlined"
            value={state.max}
            onChange={event =>
              setState(state => ({
                ...state,
                max: parseInt(event.target.value),
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
            label="Delay"
            variant="outlined"
            value={state.delay}
            onChange={event =>
              setState(state => ({
                ...state,
                delay: parseInt(event.target.value),
              }))
            }
          />
        </Grid>
      </Grid>
    </Paper>
  );
};
