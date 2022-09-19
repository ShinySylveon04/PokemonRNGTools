import React from 'react';

import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import InputLabel from '@mui/material/InputLabel';

import { useTranslation } from 'react-i18next';

export const LeadFilter = ({ state, setState }) => {
  const { t } = useTranslation();

  return (
    <FormControl fullWidth>
      <InputLabel id="lead-type-label">{t('Lead')}</InputLabel>
      <Select
        labelId="lead-type-label"
        id="lead"
        value={state.lead_filter}
        label={t('Lead')}
        onChange={event =>
          setState(state => ({
            ...state,
            lead_filter: event.target.value,
          }))
        }
      >
        <MenuItem value={0}>{t('None')}</MenuItem>
        <MenuItem value={1}>{t('Synchronize')}</MenuItem>
      </Select>
    </FormControl>
  );
};
