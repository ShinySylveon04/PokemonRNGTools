import React from 'react';
import Typography from '@mui/material/Typography';
import TextField from '@mui/material/TextField';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';

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
            fullWidth
            autoComplete="off"
            inputProps={{
              inputMode: 'text',
              maxLength: 8,
            }}
            id="seed0"
            label="Seed 0"
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
            autoComplete="off"
            fullWidth
            inputProps={{
              inputMode: 'text',
              maxLength: 8,
            }}
            id="seed1"
            label="Seed 1"
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
            autoComplete="off"
            fullWidth
            inputProps={{
              inputMode: 'text',
              maxLength: 8,
            }}
            id="seed2"
            label="Seed 2"
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
            autoComplete="off"
            fullWidth
            inputProps={{
              inputMode: 'text',
              maxLength: 8,
            }}
            id="seed3"
            label="Seed 3"
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
        <Grid item sm={6} md={3} xs={12}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="tiles"
            label="Statue Tiles"
            variant="outlined"
            value={state.tiles}
            onChange={event =>
              setState(state => ({
                ...state,
                tiles:
                  event.target.value.length === 0
                    ? ''
                    : parseInt(event.target.value),
              }))
            }
          />
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="room-label">Room Size</InputLabel>
            <Select
              labelId="room-label"
              id="room"
              value={state.large_room}
              label="Room Size"
              onChange={event =>
                setState(state => ({
                  ...state,
                  large_room: event.target.value,
                }))
              }
            >
              <MenuItem value={false}>Small</MenuItem>
              <MenuItem value={true}>Large</MenuItem>
            </Select>
          </FormControl>
        </Grid>
      </Grid>
    </Paper>
  );
};
