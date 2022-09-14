import React from 'react';
import Typography from '@mui/material/Typography';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';

import { useTranslation } from 'react-i18next';

import { IVFilters } from './IVFilters';

import { ShinyFilter } from '../../Components/ShinyFilter';
import { NatureFilter } from '../../Components/NatureFilter';
import { AbilityFilter } from '../../Components/AbilityFilter';
import { GenderRatioFilter } from '../../Components/GenderRatioFilter';

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
          <FormControl fullWidth>
            <InputLabel id="gender-label">{t('Gender')}</InputLabel>
            <Select
              labelId="gender-label"
              id="gender"
              value={state.gender_filter}
              label={t('Gender')}
              onChange={event =>
                setState(state => ({
                  ...state,
                  gender_filter: event.target.value,
                }))
              }
            >
              <MenuItem value={256}>{t('Any')}</MenuItem>
              <MenuItem value={0}>♂</MenuItem>
              <MenuItem value={254}>♀</MenuItem>
            </Select>
          </FormControl>
        </Grid>
        <IVFilters state={state} setState={setState} />
      </Grid>
    </Paper>
  );
};
