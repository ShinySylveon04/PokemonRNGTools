import React from 'react';

import InputLabel from '@mui/material/InputLabel';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';
import Grid from '@mui/material/Grid';

import { useTranslation } from 'react-i18next';

export const MethodFilter = ({ state, setState }) => {
  const { t } = useTranslation();

  return (
    <Grid item sm={6} md={3} xs={12}>
      <FormControl fullWidth>
        <InputLabel id="method-label">{t('Method')}</InputLabel>
        <Select
          labelId="method-label"
          id="method"
          value={state.method_filter}
          label={t('Method')}
          onChange={event =>
            setState(state => ({
              ...state,
              method_filter: event.target.value,
            }))
          }
        >
          <MenuItem value={1}>{t('Method H1')}</MenuItem>
          <MenuItem value={2}>{t('Method H2')}</MenuItem>
          <MenuItem value={4}>{t('Method H4')}</MenuItem>
        </Select>
      </FormControl>
    </Grid>
  );
};
