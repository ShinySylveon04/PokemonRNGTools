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
          <FormControl fullWidth>
            <InputLabel id="encounter-label">{t('Encounter Slot')}</InputLabel>
            <Select
              labelId="encounter-label"
              id="encounter-slot"
              value={state.encounter}
              label={t('Encounter Slot')}
              onChange={event =>
                setState(state => ({
                  ...state,
                  encounter: event.target.value,
                }))
              }
            >
              <MenuItem value={12}>{t('Any')}</MenuItem>
              <MenuItem value={0}>0</MenuItem>
              <MenuItem value={1}>1</MenuItem>
              <MenuItem value={2}>2</MenuItem>
              <MenuItem value={3}>3</MenuItem>
              <MenuItem value={4}>4</MenuItem>
              <MenuItem value={5}>5</MenuItem>
              <MenuItem value={6}>6</MenuItem>
              <MenuItem value={7}>7</MenuItem>
              <MenuItem value={8}>8</MenuItem>
              <MenuItem value={9}>9</MenuItem>
              <MenuItem value={10}>10</MenuItem>
              <MenuItem value={11}>11</MenuItem>
            </Select>
          </FormControl>
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <GenderRatioFilter state={state} setState={setState} />
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <GenderFilter state={state} setState={setState} />
        </Grid>
        <IVFilters state={state} setState={setState} />
      </Grid>
    </Paper>
  );
};
