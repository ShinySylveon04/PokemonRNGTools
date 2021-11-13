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
import Button from '@mui/material/Button';
import SaveAltIcon from '@mui/icons-material/SaveAlt';
import DialogTitle from '@mui/material/DialogTitle';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';

export const TrainerInfo = ({ setState, state }) => {
  const [version, setVersion] = React.useState('Sword');
  const [badges, setBadges] = React.useState(0);
  const [saveDataName, setSaveDataName] = React.useState('');
  const [saveData, setSaveData] = React.useState({
    name: 'None',
    tid: 0,
    sid: 0,
  });

  const [tid, setTid] = React.useState(0);
  const [sid, setSid] = React.useState(0);

  const handleTid = event => {
    setTid(event.target.value);
    setState(state => ({
      ...state,
      tid: parseInt(event.target.value),
    }));
  };

  console.log(saveDataName);
  console.log(saveData);

  const [open, setOpen] = React.useState(true);
  const [dialogOpen, setDialogOpen] = React.useState(false);

  const handleClick = () => {
    setOpen(!open);
  };

  const handleClickOpen = () => {
    setDialogOpen(true);
  };

  const handleClose = value => {
    setDialogOpen(false);
    // setSelectedValue(value);
  };

  const handleSaveData = () => {
    setSaveData({
      ...saveData,
      name: saveDataName,
      tid: tid,
      sid: sid,
    });
    localStorage.setItem('Profiles', JSON.stringify(saveData));
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
            <Grid item sm={6} md={3} xs={12}>
              <TextField
                fullWidth
                id="tid"
                label="TID"
                variant="outlined"
                onChange={event => handleTid(event)}
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
                  // onChange={handleChange}
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
                  // onChange={handleChange}
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
                <FormControlLabel control={<Checkbox />} label="Shiny Charm" />
              </Grid>
              <Grid item>
                <FormControlLabel control={<Checkbox />} label="Mark Charm" />
              </Grid>
            </Grid>
            <Grid item sm={3} md={3} xs={12}>
              <FormControl fullWidth>
                <InputLabel id="saved-info-label">Current Info</InputLabel>
                <Select
                  labelId="saved-info-label"
                  id="saved-info"
                  value={saveData.name}
                  label="Current Info"
                  // onChange={handleChange}
                >
                  <MenuItem value={saveData.name}>{saveData.name}</MenuItem>
                </Select>
              </FormControl>
            </Grid>
            <Grid
              container
              item
              sm={3}
              xs={12}
              sx={{ justifyContent: { xs: 'center', md: 'flex-start' } }}
            >
              <Button variant="contained" onClick={handleClickOpen}>
                Save Info <SaveAltIcon />
              </Button>
            </Grid>
            <Dialog open={dialogOpen} onClose={handleClose}>
              <DialogTitle>Name Profile</DialogTitle>
              <DialogContent>
                <DialogContentText>
                  Type in the name of the profile.
                </DialogContentText>
                <TextField
                  autoFocus
                  margin="dense"
                  id="saveDataName"
                  label="Profile Name"
                  type="text"
                  fullWidth
                  variant="standard"
                  onChange={event => setSaveDataName(event.target.value)}
                />
              </DialogContent>
              <DialogActions>
                <Button onClick={handleSaveData}>Save</Button>
              </DialogActions>
            </Dialog>
          </Grid>
        </Collapse>
      </Grid>
    </Paper>
  );
};
