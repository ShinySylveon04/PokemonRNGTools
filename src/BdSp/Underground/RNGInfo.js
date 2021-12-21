import React from 'react';
import Typography from '@mui/material/Typography';
import TextField from '@mui/material/TextField';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';
import Checkbox from '@mui/material/Checkbox';
import FormControlLabel from '@mui/material/FormControlLabel';

export const RNGInfo = ({ setState, state }) => {
  const [checked, setChecked] = React.useState(false);
  const handleChange = event => {
    setState({ ...state, diglett_boost: event.target.checked });
    setChecked(event.target.checked);
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
        <Grid item sm={6} md={3} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="room-name-label">Room Name</InputLabel>
            <Select
              labelId="room-name-label"
              id="room-name"
              value={state.room_name}
              label="Room Name"
              onChange={event =>
                setState(state => ({
                  ...state,
                  room_name: event.target.value,
                }))
              }
            >
              <MenuItem value={2}>Spacious Cave</MenuItem>
              <MenuItem value={3}>Grassland Cave</MenuItem>
              <MenuItem value={4}>Fountainspring Cave</MenuItem>
              <MenuItem value={5}>Rocky Cave</MenuItem>
              <MenuItem value={6}>Volcanic Cave</MenuItem>
              <MenuItem value={7}>Swampy Cave</MenuItem>
              <MenuItem value={8}>Dazzling Cave</MenuItem>
              <MenuItem value={9}>Whiteout Cave</MenuItem>
              <MenuItem value={10}>Icy Cave</MenuItem>
              <MenuItem value={11}>Riverbank Cave</MenuItem>
              <MenuItem value={12}>Sandsear Cave</MenuItem>
              <MenuItem value={13}>Still-Water Cavern</MenuItem>
              <MenuItem value={14}>Sunlit Cavern</MenuItem>
              <MenuItem value={15}>Big Bluff Cavern</MenuItem>
              <MenuItem value={16}>Stargleam Cavern</MenuItem>
              <MenuItem value={17}>Glacial Cavern</MenuItem>
              <MenuItem value={18}>Bogsunk Cavern</MenuItem>
              <MenuItem value={19}>Typhlo Cavern</MenuItem>
            </Select>
          </FormControl>
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="version-label">Game Version</InputLabel>
            <Select
              labelId="version-label"
              id="version"
              value={state.version}
              label="Game Version"
              onChange={event =>
                setState(state => ({
                  ...state,
                  version: event.target.value,
                }))
              }
            >
              <MenuItem value={'DIAMOND'}>Brilliant Diamond</MenuItem>
              <MenuItem value={'PEARL'}>Shining Pearl</MenuItem>
            </Select>
          </FormControl>
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="progress-label">Game Progress</InputLabel>
            <Select
              labelId="progress-label"
              id="progress"
              value={state.game_progress}
              label="Game Progress"
              onChange={event =>
                setState(state => ({
                  ...state,
                  game_progress: event.target.value,
                }))
              }
            >
              <MenuItem value={1}>Underground Unlocked</MenuItem>
              <MenuItem value={2}>Stregth Obtained</MenuItem>
              <MenuItem value={3}>Defog Obtained</MenuItem>
              <MenuItem value={4}>7 Badges</MenuItem>
              <MenuItem value={5}>Waterfall Obtained</MenuItem>
              <MenuItem value={6}>National Dex Obtained</MenuItem>
            </Select>
          </FormControl>
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
              label="Diglett Boost"
            />
          </Grid>
        </Grid>
      </Grid>
    </Paper>
  );
};
