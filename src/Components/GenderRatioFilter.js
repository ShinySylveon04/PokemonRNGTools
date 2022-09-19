import React from 'react';

import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';

import { useTranslation } from 'react-i18next';

export const GenderRatioFilter = ({ state, setState }) => {
  const { t } = useTranslation();

  return (
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
  );
};
