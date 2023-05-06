import React from 'react';
import TextField from '@mui/material/TextField';
import Checkbox from '@mui/material/Checkbox';
import FormControlLabel from '@mui/material/FormControlLabel';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';
import { useTranslation } from 'react-i18next';
import { useFormikContext } from 'formik';

type SharedConfig = {
  id: string;
  label: string;
  defaultValue: string;
};

export type FieldConfig = SharedConfig &
  (
    | {
        type: 'select';
        options: string[];
      }
    | {
        type: 'hex_number' | 'number' | 'text' | 'checkbox';
        options?: never;
      }
  );

type Props = FieldConfig;

export const InputField = ({
  id,
  type,
  label: nonTranslatedLabel,
  options,
}: Props) => {
  const { t } = useTranslation();
  const label = t(nonTranslatedLabel);
  const formik = useFormikContext<Record<string, unknown>>();

  const value = formik.values[id];
  const error = formik.touched[id] && Boolean(formik.errors[id]);
  const helperText = formik.touched[id] && formik.errors[id];

  if (type === 'checkbox') {
    return (
      <FormControlLabel
        sx={{ userSelect: 'none' }}
        label={label}
        control={
          <Checkbox
            id={id}
            checked={!!value}
            onChange={formik.handleChange}
            inputProps={{ 'aria-label': label }}
          />
        }
      />
    );
  }

  if (type === 'number') {
    return (
      <TextField
        fullWidth
        required
        id={id}
        label={label}
        variant="outlined"
        type="number"
        autoComplete="off"
        onChange={formik.handleChange}
        value={value}
        error={error}
        helperText={helperText}
      />
    );
  }

  if (type === 'text') {
    return (
      <TextField
        fullWidth
        required
        id={id}
        label={label}
        variant="outlined"
        autoComplete="off"
        onChange={formik.handleChange}
        value={value}
        error={error}
        helperText={helperText}
      />
    );
  }

  if (type === 'hex_number') {
    return (
      <TextField
        fullWidth
        required
        id={id}
        label={label}
        variant="outlined"
        autoComplete="off"
        onChange={formik.handleChange}
        value={value}
        error={error}
        helperText={helperText}
      />
    );
  }

  if (type === 'select') {
    return (
      <TextField
        select
        fullWidth
        required
        id={id}
        name={id}
        label={label}
        autoComplete="off"
        onChange={formik.handleChange}
        value={value}
        error={error}
      >
        {options.map(option => (
          <MenuItem key={option} value={option}>
            {option}
          </MenuItem>
        ))}
      </TextField>
    );
  }
};
