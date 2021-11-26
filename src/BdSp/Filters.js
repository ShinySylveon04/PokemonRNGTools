import React from 'react';
import Typography from '@mui/material/Typography';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';

import { natures } from '../natures';

export const Filters = ({ setState, state }) => {
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
            Filters
          </Typography>
        </Grid>
        <Grid item sm={6} md={4} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="shiny-type-label">Shiny</InputLabel>
            <Select
              labelId="shiny-type-label"
              id="shiny-type"
              value={state.shiny}
              label="Shiny"
              onChange={event =>
                setState(state => ({
                  ...state,
                  shiny: event.target.value,
                }))
              }
            >
              <MenuItem value={false}>None</MenuItem>
              <MenuItem value={true}>Any</MenuItem>
            </Select>
          </FormControl>
        </Grid>
        <Grid item sm={6} md={4} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="nature-label">Nature</InputLabel>
            <Select
              labelId="nature-label"
              id="nature"
              value={state.nature}
              label="Nature"
              onChange={event =>
                setState(state => ({
                  ...state,
                  nature: event.target.value,
                }))
              }
            >
              <MenuItem value={25}>Any</MenuItem>
              {natures.map((nature, index) => (
                <MenuItem value={index} key={nature}>
                  {nature}
                </MenuItem>
              ))}
            </Select>
          </FormControl>
        </Grid>
        <Grid item sm={6} md={4} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="ability-label">Ability</InputLabel>
            <Select
              labelId="ability-label"
              id="ability"
              value={state.ability}
              label="Ability"
              onChange={event =>
                setState(state => ({
                  ...state,
                  ability: event.target.value,
                }))
              }
            >
              <MenuItem value={3}>Any</MenuItem>
              <MenuItem value={0}>0</MenuItem>
              <MenuItem value={1}>1</MenuItem>
            </Select>
          </FormControl>
        </Grid>
      </Grid>
    </Paper>
  );
};
