import React from 'react';

import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';

import { useTranslation } from 'react-i18next';

export const ShinyFilter = ({ state, setState }) => {
  const { t } = useTranslation();
  return (
    <React.Fragment>
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
    </React.Fragment>
  );
};
