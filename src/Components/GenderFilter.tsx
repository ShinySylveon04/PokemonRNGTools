import React from 'react';

import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';

import { useTranslation } from 'react-i18next';

export const GenderFilter = ({ state, setState }) => {
  const { t } = useTranslation();

  return (
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
  );
};
