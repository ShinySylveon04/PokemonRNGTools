import React from 'react';
import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';

import { useTranslation } from 'react-i18next';

import { IVFilter } from '../../Components/IVFilter';
import { ShinyFilter } from '../../Components/ShinyFilter';
import { NatureFilter } from '../../Components/NatureFilter';
import { AbilityFilter } from '../../Components/AbilityFilter';
import { GenderRatioFilter } from '../../Components/GenderRatioFilter';
import { GenderFilter } from '../../Components/GenderFilter';

export const Filters = ({ setState, state }) => {
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
            {t('Filters')}
          </Typography>
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <ShinyFilter state={state} setState={setState} />
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <NatureFilter state={state} setState={setState} />
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <AbilityFilter state={state} setState={setState} />
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <GenderRatioFilter state={state} setState={setState} />
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <GenderFilter state={state} setState={setState} />
        </Grid>
        <IVFilter state={state} setState={setState} />
      </Grid>
    </Paper>
  );
};
