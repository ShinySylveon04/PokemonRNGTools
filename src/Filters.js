import React from 'react';
import Typography from '@mui/material/Typography';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';

export const Filters = ({setShiny, shiny}) => {
  // const [shiny, setShiny] = React.useState(0);
  const [mark, setMark] = React.useState('None');
  const [slot, setSlot] = React.useState('Any');

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
        <Typography variant="h6" align="left" color='primary'>
          Filters
        </Typography>
      </Grid>
      <Grid item sm={6} md={4} xs={12}>
        <FormControl fullWidth>
          <InputLabel id="shiny-type-label">Shiny</InputLabel>
          <Select
            labelId="shiny-type-label"
            id="shiny-type"
            value={shiny}
            label="Shiny"
            onChange={event => setShiny(event.target.value)}
          >
            <MenuItem value={0}>None</MenuItem>
            <MenuItem value={1}>Any</MenuItem>
            <MenuItem value={2}>Star</MenuItem>
            <MenuItem value={3}>Square</MenuItem>
          </Select>
        </FormControl>
      </Grid>
      <Grid item sm={6} md={4} xs={12}>
        <FormControl fullWidth>
          <InputLabel id="mark-type-label">Mark</InputLabel>
          <Select
            labelId="mark-type-label"
            id="mark-type"
            value={mark}
            label="Mark"
            onChange={handleChange}
          >
            <MenuItem value={'None'}>None</MenuItem>
            <MenuItem value={'Rare'}>Rare</MenuItem>
            <MenuItem value={'Other'}>Other</MenuItem>
          </Select>
        </FormControl>
      </Grid>
      <Grid item sm={6} md={4} xs={12}>
        <FormControl fullWidth>
          <InputLabel id="slot-label">Encounter Slot</InputLabel>
          <Select
            labelId="slot-label"
            id="slot"
            value={slot}
            label="Encounter Slot"
            onChange={handleChange}
          >
            <MenuItem value={'Any'}>Any</MenuItem>
            <MenuItem value={1}>1</MenuItem>
          </Select>
        </FormControl>
      </Grid>
    </Grid>
    </Paper>
  );
};
