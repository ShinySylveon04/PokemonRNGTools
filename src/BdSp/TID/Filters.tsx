import React from 'react';
import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';
import TextField from '@mui/material/TextField';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import InputLabel from '@mui/material/InputLabel';

import { useTranslation } from 'react-i18next';

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
          <FormControl fullWidth>
            <InputLabel id="id-type-label">{t('ID Filter')}</InputLabel>
            <Select
              labelId="id-type-label"
              id="id-type"
              value={state.id_filter}
              label={t('ID Filter')}
              onChange={event =>
                setState(state => ({
                  ...state,
                  id_filter: event.target.value,
                }))
              }
            >
              <MenuItem value={'None'}>{t('None')}</MenuItem>
              <MenuItem value={'TID'}>{t('TID')}</MenuItem>
              <MenuItem value={'SID'}>{t('SID')}</MenuItem>
              <MenuItem value={'TSV'}>{t('TSV')}</MenuItem>
              <MenuItem value={'G8TID'}>{t('Gen 8 TID')}</MenuItem>
            </Select>
          </FormControl>
        </Grid>
        <Grid item sm={6} md={3} xs={12}>
          <TextField
            multiline
            fullWidth
            autoComplete="off"
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            id="id"
            label=""
            variant="outlined"
            onChange={event => {
              setState(state => ({
                ...state,
                id: event.target.value,
              }));
            }}
          />
        </Grid>
      </Grid>
    </Paper>
  );
};
