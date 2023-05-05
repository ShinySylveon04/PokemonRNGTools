import React from 'react';
import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';

import { useTranslation } from 'react-i18next';

import { RNGStates } from '../../Components/RNGStates';
import { RNGAdvances } from '../../Components/RNGAdvances';

export const RNGInfo = ({ setState, state }) => {
  const { t } = useTranslation();

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
        <RNGStates state={state} setState={setState} />
        <RNGAdvances state={state} setState={setState} />
      </Grid>
    </Paper>
  );
};
