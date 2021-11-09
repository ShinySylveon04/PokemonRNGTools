import React from 'react';
import Typography from '@mui/material/Typography';
import TextField from '@mui/material/TextField';
import Checkbox from '@mui/material/Checkbox';
import FormControlLabel from '@mui/material/FormControlLabel';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';
import Collapse from '@mui/material/Collapse';
import ExpandLess from '@mui/icons-material/ExpandLess';
import ExpandMore from '@mui/icons-material/ExpandMore';

export const TrainerInfo = ({ setState }) => {
  const [version, setVersion] = React.useState('Sword');
  const [badges, setBadges] = React.useState(0);

  const [open, setOpen] = React.useState(true);

  const handleClick = () => {
    setOpen(!open);
  };

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
        <Grid item sm={11} xs={10}>
          <Typography variant="h6" align="left" color="primary">
            Trainer Info
          </Typography>
        </Grid>
        <Grid item sm={1} xs={2} onClick={handleClick}>
          {' '}
          {open ? (
            <ExpandLess color="primary" sx={{ margin: '5px' }} />
          ) : (
            <ExpandMore color="primary" sx={{ margin: '5px' }} />
          )}
        </Grid>
        <Collapse in={open} timeout="auto" unmountOnExit sx={{ width: '90%' }}>
          <Grid
            container
            spacing={2}
            direction="row"
            justifyContent="center"
            alignItems="center"
          >
            <Grid item sm={3} xs={6}>
              <TextField
                id="tsv"
                label="TSV"
                variant="outlined"
                onChange={event =>
                  setState(state => ({
                    ...state,
                    tsv: parseInt(event.target.value),
                  }))
                }
              />
            </Grid>
            <Grid item sm={3} xs={6}>
              <FormControl fullWidth>
                <InputLabel id="game-version-label">Game Version</InputLabel>
                <Select
                  labelId="game-version-label"
                  id="game-version"
                  value={version}
                  label="Game Version"
                  onChange={handleChange}
                >
                  <MenuItem value={'Sword'}>Sword</MenuItem>
                  <MenuItem value={'Shield'}>Shield</MenuItem>
                </Select>
              </FormControl>
            </Grid>
            <Grid item sm={3} xs={6}>
              <FormControl fullWidth>
                <InputLabel id="badge-count-label">Badge Count</InputLabel>
                <Select
                  labelId="badge-count-label"
                  id="badge-count"
                  value={badges}
                  label="Badge Count"
                  onChange={handleChange}
                >
                  <MenuItem value={0}>0</MenuItem>
                  <MenuItem value={1}>1</MenuItem>
                </Select>
              </FormControl>
            </Grid>
            <Grid item sm={3} xs={6}>
              <Grid item>
                <FormControlLabel control={<Checkbox />} label="Shiny Charm" />
              </Grid>
              <Grid item>
                <FormControlLabel control={<Checkbox />} label="Mark Charm" />
              </Grid>
            </Grid>
          </Grid>
        </Collapse>
      </Grid>
    </Paper>
  );
};
