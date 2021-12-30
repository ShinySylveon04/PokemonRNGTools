import React from 'react';
import Typography from '@mui/material/Typography';
import TextField from '@mui/material/TextField';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';
import Checkbox from '@mui/material/Checkbox';
import FormControlLabel from '@mui/material/FormControlLabel';

export const RNGInfo = ({ setState, state }) => {
  const [checked, setChecked] = React.useState(true);
  const handleChange = event => {
    setState({ ...state, set_ivs: event.target.checked });
    setChecked(event.target.checked);
  };

  const handlePaste = event => {
    const text = event.clipboardData.getData('Text').split('\n');

    setState(state => ({
      ...state,
      state0: text[0],
      state1: text[1],
      state2: text[2],
      state3: text[3],
    }));
  };

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
            onPaste={event => handlePaste(event)}
            fullWidth
            autoComplete="off"
            inputProps={{
              inputMode: 'text',
              maxLength: 8,
            }}
            value={state.state0}
            id="seed0"
            label="Seed 0"
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
            label="Seed 1"
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
            label="Seed 2"
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
            label="Seed 3"
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                state3: event.target.value,
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
                min:
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
            label="Max Advances"
            variant="outlined"
            value={state.max}
            onChange={event =>
              setState(state => ({
                ...state,
                max:
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
            label="Delay"
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
        <Grid
          container
          item
          sm={6}
          md={3}
          xs={12}
          sx={{ justifyContent: { xs: 'center', md: 'flex-start' } }}
        >
          <Grid item>
            <FormControlLabel
              control={
                <Checkbox
                  checked={checked}
                  onChange={handleChange}
                  inputProps={{ 'aria-label': 'controlled' }}
                />
              }
              label="Set IVs"
            />
          </Grid>
        </Grid>
      </Grid>
    </Paper>
  );
};
