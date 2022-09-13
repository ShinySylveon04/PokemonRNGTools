import React from 'react';

import InputLabel from '@mui/material/InputLabel';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';

import { useTranslation } from 'react-i18next';

export const AbilityFilter = ({ state, setState }) => {
  const { t } = useTranslation();

  return (
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
  );
};
