import React from 'react';

import Checkbox from '@mui/material/Checkbox';
import ListItemText from '@mui/material/ListItemText';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';

import { useTranslation } from 'react-i18next';
import { natureOptions } from '../natures';

export const NatureFilter = ({ state, setState }) => {
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
  );
};
