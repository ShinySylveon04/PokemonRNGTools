import React from 'react';
import Typography from '@mui/material/Typography';
import TextField from '@mui/material/TextField';
import Grid from '@mui/material/Grid';
import Checkbox from '@mui/material/Checkbox';
import FormControlLabel from '@mui/material/FormControlLabel';
import Paper from '@mui/material/Paper';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';

export const RNGInfo = ({
  setState,
  state,
  state0Error,
  state1Error,
  setState0Error,
  setState1Error,
}) => {
  const handleChange = event => {
    setEncounter(event.target.value);
  };

  const regex = /[0-9A-Fa-f]{16}/;

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
            fullWidth
            error={state0Error.error}
            helperText={state0Error.text}
            inputProps={{
              inputMode: 'text',
              maxLength: 16,
            }}
            id="state0"
            label="State 0"
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                state0: !regex.test(event.target.value)
                  ? ''
                  : BigInt(`0x${event.target.value}`),
              }));
            }}
          />
        </Grid>
        <Grid container item sm={6} xs={12} justifyContent="center">
          <TextField
            fullWidth
            error={state1Error.error}
            helperText={state1Error.text}
            inputProps={{
              inputMode: 'text',
              maxLength: 16,
            }}
            id="state1"
            label="State 1"
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                state1: !regex.test(event.target.value)
                  ? ''
                  : BigInt(`0x${event.target.value}`),
              }));
            }}
          />
        </Grid>
        <Grid item sm={6} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="encounter-type-label">Encounter Type</InputLabel>
            <Select
              labelId="encounter-type-label"
              id="encounter-type"
              value={state.encounter}
              label="Encounter Type"
              onChange={event =>
                setState(state => ({
                  ...state,
                  encounter: event.target.value,
                }))
              }
            >
              <MenuItem value={0}>Static</MenuItem>
              <MenuItem value={1}>Dynamic</MenuItem>
            </Select>
          </FormControl>
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
        {/* <Grid container item sm={6} xs={12} sx={{justifyContent: {xs: "center", md: 'flex-start'}}}>
        <Grid item>
          <FormControlLabel control={<Checkbox />} label="Weather Active" />
        </Grid>
        <Grid item>
          <FormControlLabel control={<Checkbox />} label="Fishing" />
        </Grid>
      </Grid> */}
      </Grid>
    </Paper>
  );
};
