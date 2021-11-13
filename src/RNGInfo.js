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

export const RNGInfo = ({ setState, state }) => {
  const handleChange = event => {
    setEncounter(event.target.value);
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
            fullWidth
            id="state0"
            label="State 0"
            variant="outlined"
            onChange={event =>
              setState(state => ({
                ...state,
                state0: BigInt(`0x${event.target.value}`),
              }))
            }
          />
        </Grid>
        <Grid container item sm={6} xs={12} justifyContent="center">
          <TextField
            fullWidth
            id="state1"
            label="State 1"
            variant="outlined"
            onChange={event =>
              setState(state => ({
                ...state,
                state1: BigInt(`0x${event.target.value}`),
              }))
            }
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
