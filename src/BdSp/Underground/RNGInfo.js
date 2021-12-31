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

import { useTranslation } from 'react-i18next';

import { setRngStateFromClipboard } from '../../Utils/setRngState';

export const RNGInfo = ({ setState, state }) => {
  const [checked, setChecked] = React.useState(false);
  const handleChange = event => {
    setState({ ...state, diglett_boost: event.target.checked });
    setChecked(event.target.checked);
  };

  const { t } = useTranslation();

  const handlePaste = event => setRngStateFromClipboard(event, setState);
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
            {t('RNG Info')}
          </Typography>
        </Grid>
        <Grid container item sm={6} xs={12} justifyContent="center">
          <TextField
            onPaste={event => handlePaste(event)}
            fullWidth
            autoComplete="off"
            inputProps={{
              inputMode: 'text',
              maxLength: 8,
            }}
            value={state.state0}
            id="seed0"
            label={t('Seed 0')}
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                state0: event.target.value,
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
            value={state.state1}
            id="seed1"
            label={t('Seed 1')}
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                state1: event.target.value,
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
            value={state.state2}
            id="seed2"
            label={t('Seed 2')}
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                state2: event.target.value,
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
            value={state.state3}
            id="seed3"
            label={t('Seed 3')}
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                state3: event.target.value,
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
            label={t('Min Advances')}
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
            label={t('Max Advances')}
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
            label={t('Delay')}
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
            label={t('Statue Tiles')}
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
            <InputLabel id="room-label">{t('Room Size')}</InputLabel>
            <Select
              labelId="room-label"
              id="room"
              value={state.large_room}
              label={t('Room Size')}
              onChange={event =>
                setState(state => ({
                  ...state,
                  large_room: event.target.value,
                }))
              }
            >
              <MenuItem value={false}>{t('Small')}</MenuItem>
              <MenuItem value={true}>{t('Large')}</MenuItem>
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
              label={t('Diglett Boost')}
            />
          </Grid>
        </Grid>
      </Grid>
    </Paper>
  );
};
