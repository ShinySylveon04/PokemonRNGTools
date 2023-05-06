import React from 'react';
import TextField from '@mui/material/TextField';
import Checkbox from '@mui/material/Checkbox';
import FormControlLabel from '@mui/material/FormControlLabel';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';
import { useTranslation } from 'react-i18next';

type SelectInput = {
  type: 'select';
  label: string;
  options: string[];
};

type HexNumberInput = {
  type: 'hex_number';
  label: string;
};

type NumberInput = {
  type: 'number';
  label: string;
};

type CheckboxInput = {
  type: 'checkbox';
  label: string;
};

export type Props =
  | {
      type: SelectInput['type'];
      config: Omit<SelectInput, 'type'>;
      defaultValue?: string;
      onChange: (newValue: string) => void;
    }
  | {
      type: NumberInput['type'];
      config: Omit<NumberInput, 'type'>;
      defaultValue?: number;
      onChange: (newValue: number) => void;
    }
  | {
      type: HexNumberInput['type'];
      config: Omit<HexNumberInput, 'type'>;
      defaultValue?: number;
      onChange: (newValue: number) => void;
    }
  | {
      type: CheckboxInput['type'];
      config: Omit<CheckboxInput, 'type'>;
      defaultValue?: boolean;
      onChange: (newValue: boolean) => void;
    };

export const InputField = ({ type, config, defaultValue, onChange }: Props) => {
  const { t } = useTranslation();
  const label = t(config.label);

  if (type === 'checkbox') {
    return (
      <FormControlLabel
        sx={{ userSelect: 'none' }}
        control={
          <Checkbox
            defaultChecked={defaultValue}
            onChange={event => onChange(event.target.checked)}
            inputProps={{ 'aria-label': label }}
          />
        }
        label={label}
      />
    );
  }

  if (type === 'number') {
    return (
      <TextField
        fullWidth
        label={label}
        variant="outlined"
        type="number"
        defaultValue={defaultValue}
        onChange={event => onChange(parseInt(event.target.value, 10))}
      />
    );
  }

  if (type === 'hex_number') {
    return (
      <TextField
        fullWidth
        label={label}
        variant="outlined"
        defaultValue={defaultValue?.toString(16)}
        onChange={event => onChange(parseInt(event.target.value, 16))}
      />
    );
  }

  if (type === 'select') {
    return (
      <Select
        fullWidth
        label={label}
        defaultValue={defaultValue}
        onChange={event => onChange(event.target.value)}
      >
        {config.options.map(option => (
          <MenuItem key={option} value={option}>
            {option}
          </MenuItem>
        ))}
      </Select>
    );
  }
};
