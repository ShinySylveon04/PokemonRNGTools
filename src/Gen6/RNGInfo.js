import React from 'react';
import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';
import TextField from '@mui/material/TextField';
import Checkbox from '@mui/material/Checkbox';
import FormControlLabel from '@mui/material/FormControlLabel';

import { useTranslation } from 'react-i18next';

import { RNGAdvances } from '../Components/RNGAdvances';
import { RNGDelay } from '../Components/RNGDelay';

export const RNGInfo = ({ setState, state }) => {
  const { t } = useTranslation();

  const [checked, setChecked] = React.useState(false);
  const handleChange = event => {
    setState({ ...state, iv_rolls: event.target.checked });
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
            {t('RNG Info')}
          </Typography>
        </Grid>
        <Grid container item sm={3} xs={12} justifyContent="center">
          <TextField
            fullWidth
            autoComplete="off"
            inputProps={{
              inputMode: 'text',
              maxLength: 8,
            }}
            value={state.rng_state}
            id="seed"
            label={t('Seed')}
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                rng_state: event.target.value,
              }));
            }}
          />
        </Grid>
        <RNGAdvances state={state} setState={setState} />
        <RNGDelay state={state} setState={setState} />
        <Grid item sm={3} xs={12}>
          <FormControlLabel
            control={
              <Checkbox
                checked={checked}
                onChange={handleChange}
                inputProps={{ 'aria-label': 'controlled' }}
              />
            }
            label={t('Mew or Celebi')}
          />
        </Grid>
      </Grid>
    </Paper>
  );
};
