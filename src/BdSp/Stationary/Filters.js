import React from 'react';
import Typography from '@mui/material/Typography';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';
import Checkbox from '@mui/material/Checkbox';
import ListItemText from '@mui/material/ListItemText';

import { useTranslation } from 'react-i18next';

import { natureOptions } from '../../natures';
import { IVFilters } from './IVFilters';

export const Filters = ({ setState, state }) => {
  const { t } = useTranslation();

  const handleNatureChange = event => {
    // If Any is selected, then a nature is selected, deselect Any
    if (state.nature_filter.includes(25) && event.target.value.length > 1) {
      const index = event.target.value.indexOf(25);
      if (index > -1) {
        event.target.value.splice(index, 1);
      }
      setState(state => ({
        ...state,
        nature_filter: event.target.value,
      }));
    }
    // If nature(s) are selected, then select Any, deselect all other natures
    // Or if all natures are unselected set to Any by default
    else if (
      (!state.nature_filter.includes(25) && event.target.value.includes(25)) ||
      event.target.value.length === 0
    ) {
      setState(state => ({
        ...state,
        nature_filter: [25],
      }));
    }
    // Otherwise, add natures selected
    else {
      setState(state => ({
        ...state,
        nature_filter: event.target.value,
      }));
    }
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
            {t('Filters')}
          </Typography>
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="shiny-type-label">{t('Shiny')}</InputLabel>
            <Select
              labelId="shiny-type-label"
              id="shiny-type"
              value={state.shiny_filter}
              label={t('Shiny')}
              onChange={event =>
                setState(state => ({
                  ...state,
                  shiny_filter: event.target.value,
                }))
              }
            >
              <MenuItem value={4}>{t('Any')}</MenuItem>
              <MenuItem value={1}>{t('Star')}</MenuItem>
              <MenuItem value={2}>{t('Square')}</MenuItem>
              <MenuItem value={3}>{t('Star/Square')}</MenuItem>
            </Select>
          </FormControl>
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="nature-label">{t('Nature')}</InputLabel>
            <Select
              multiple
              labelId="nature-label"
              id="nature"
              value={state.nature_filter}
              label={t('Nature')}
              renderValue={selected =>
                selected
                  .map(nature =>
                    t(
                      `nature.${
                        natureOptions.find(
                          natureOption => natureOption.value === nature,
                        ).name
                      }`,
                    ),
                  )
                  .join(', ')
              }
              onChange={event => handleNatureChange(event)}
            >
              {natureOptions.map(nature => (
                <MenuItem value={nature.value} key={nature.value}>
                  <Checkbox
                    checked={state.nature_filter.indexOf(nature.value) > -1}
                  />
                  <ListItemText primary={t(`nature.${nature.name}`)} />
                </MenuItem>
              ))}
            </Select>
          </FormControl>
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="ability-label">{t('Ability')}</InputLabel>
            <Select
              labelId="ability-label"
              id="ability"
              value={state.ability_filter}
              label={t('Ability')}
              onChange={event =>
                setState(state => ({
                  ...state,
                  ability_filter: event.target.value,
                }))
              }
            >
              <MenuItem value={3}>{t('Any')}</MenuItem>
              <MenuItem value={0}>0</MenuItem>
              <MenuItem value={1}>1</MenuItem>
            </Select>
          </FormControl>
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <FormControl fullWidth>
            <InputLabel id="genderRatio-label">{t('Gender Ratio')}</InputLabel>
            <Select
              labelId="genderRatio-label"
              id="genderRatio"
              value={state.gender_ratio}
              label={t('Gender Ratio')}
              onChange={event =>
                setState(state => ({
                  ...state,
                  gender_ratio: event.target.value,
                }))
              }
            >
              <MenuItem value={256}>{t('No Set Gender')}</MenuItem>
              <MenuItem value={255}>{t('Genderless')}</MenuItem>
              <MenuItem value={127}>50% ♂ / 50% ♀</MenuItem>
              <MenuItem value={191}>25% ♂ / 75% ♀</MenuItem>
              <MenuItem value={63}>75% ♂ / 25% ♀</MenuItem>
              <MenuItem value={31}>87.5% ♂ / 12.5% ♀</MenuItem>
              <MenuItem value={0}>100% ♂</MenuItem>
              <MenuItem value={254}>100% ♀</MenuItem>
            </Select>
          </FormControl>
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
