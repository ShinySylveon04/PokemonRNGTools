import React from 'react';
import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';
import TextField from '@mui/material/TextField';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import InputLabel from '@mui/material/InputLabel';

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
        <Grid item sm={6} md={3} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="id-type-label">ID Filter</InputLabel>
            <Select
              labelId="id-type-label"
              id="id-type"
              value={state.id_filter}
              label="ID Filter"
              onChange={event =>
                setState(state => ({
                  ...state,
                  id_filter: event.target.value,
                }))
              }
            >
              <MenuItem value={'None'}>None</MenuItem>
              <MenuItem value={'TID'}>TID</MenuItem>
              <MenuItem value={'SID'}>SID</MenuItem>
              <MenuItem value={'TSV'}>TSV</MenuItem>
              <MenuItem value={'G8TID'}>Gen 8 TID</MenuItem>
            </Select>
          </FormControl>
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <TextField
            multiline
            fullWidth
            autoComplete="off"
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            id="id"
            label=""
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                id: event.target.value,
              }));
            }}
          />
        </Grid>
      </Grid>
    </Paper>
  );
};
