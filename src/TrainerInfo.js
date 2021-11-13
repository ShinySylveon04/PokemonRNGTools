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

import { SaveInfoDialog } from './SaveInfoDialog';
import { SaveProfile } from './SaveProfile';
import { ProfileSelect } from './ProfileSelect';

export const TrainerInfo = ({ setState, state }) => {
  const [version, setVersion] = React.useState('Sword');
  const [badges, setBadges] = React.useState(0);
  const [saveProfile, setProfile] = React.useState({
    name: 'No Profile',
    tid: 0,
    sid: 0,
  });

  const [checked, setChecked] = React.useState(false);

  const handleChange = event => {
    setState({ ...state, shiny_charm: event.target.checked });
    setChecked(event.target.checked);
  };

  const [collapseOpen, setCollapseOpen] = React.useState(true);
  const [dialogOpen, setDialogOpen] = React.useState(false);

  const handleCollaspe = () => {
    setCollapseOpen(!collapseOpen);
  };

  const handleDialogOpen = () => {
    setDialogOpen(true);
  };

  const handleDialogClose = value => {
    setDialogOpen(false);
  };

  const handleSaveProfile = () => {
    setProfile({
      ...saveProfile,
      name: 'test',
      tid: state.tid,
      sid: state.sid,
    });
    localStorage.setItem('Profiles', JSON.stringify(saveProfile));
    setDialogOpen(false);
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
        <Grid item sm={1} xs={2} onClick={handleCollaspe}>
          {collapseOpen ? (
            <ExpandLess color="primary" sx={{ margin: '5px' }} />
          ) : (
            <ExpandMore color="primary" sx={{ margin: '5px' }} />
          )}
        </Grid>
        <Collapse
          in={collapseOpen}
          timeout="auto"
          unmountOnExit
          sx={{ width: '90%' }}
        >
          <Grid
            container
            spacing={2}
            direction="row"
            justifyContent="center"
            alignItems="center"
          >
            <Grid item sm={6} md={3} xs={12}>
              <TextField
                fullWidth
                id="tid"
                label="TID"
                variant="outlined"
                onChange={event =>
                  setState(state => ({
                    ...state,
                    tid: parseInt(event.target.value),
                  }))
                }
              />
            </Grid>
            <Grid item sm={6} md={3} xs={12}>
              <TextField
                fullWidth
                id="sid"
                label="SID"
                variant="outlined"
                onChange={event =>
                  setState(state => ({
                    ...state,
                    sid: parseInt(event.target.value),
                  }))
                }
              />
            </Grid>
            <Grid item sm={6} md={3} xs={12}>
              <FormControl fullWidth>
                <InputLabel id="game-version-label">Game Version</InputLabel>
                <Select
                  labelId="game-version-label"
                  id="game-version"
                  value={version}
                  label="Game Version"
                >
                  <MenuItem value={'Sword'}>Sword</MenuItem>
                  <MenuItem value={'Shield'}>Shield</MenuItem>
                </Select>
              </FormControl>
            </Grid>
            <Grid item sm={6} md={3} xs={12}>
              <FormControl fullWidth>
                <InputLabel id="badge-count-label">Badge Count</InputLabel>
                <Select
                  labelId="badge-count-label"
                  id="badge-count"
                  value={badges}
                  label="Badge Count"
                >
                  <MenuItem value={0}>0</MenuItem>
                  <MenuItem value={1}>1</MenuItem>
                </Select>
              </FormControl>
            </Grid>
            <Grid
              container
              item
              sm={6}
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
                  label="Shiny Charm"
                />
              </Grid>
              <Grid item>
                <FormControlLabel control={<Checkbox />} label="Mark Charm" />
              </Grid>
            </Grid>
            <ProfileSelect saveProfile={saveProfile} />
            <SaveProfile handleDialogOpen={handleDialogOpen} />
            <SaveInfoDialog
              dialogOpen={dialogOpen}
              handleDialogClose={handleDialogClose}
              handleSaveProfile={handleSaveProfile}
            />
          </Grid>
        </Collapse>
      </Grid>
    </Paper>
  );
};
