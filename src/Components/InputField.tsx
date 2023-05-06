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
};

export type Props = SharedConfig &
  (
    | {
        type: 'select';
        defaultValue?: string;
        options: string[];
      }
    | {
        type: 'hex_number';
        defaultValue?: number;
        options?: never;
      }
    | {
        type: 'number';
        defaultValue?: number;
        options?: never;
      }
    | {
        type: 'checkbox';
        defaultValue?: boolean;
        options?: never;
      }
  );

export const InputField = ({
  id,
  type,
  defaultValue,
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
            defaultChecked={defaultValue ?? false}
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
        defaultValue={defaultValue}
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
        defaultValue={defaultValue?.toString(16)}
        onChange={formik.handleChange}
        value={value}
        error={error}
        helperText={helperText}
      />
    );
  }

  if (type === 'select') {
    return (
      <Select
        fullWidth
        required
        id={id}
        label={label}
        autoComplete="off"
        defaultValue={defaultValue}
        onChange={formik.handleChange}
        value={value}
        error={error}
      >
        {options.map(option => (
          <MenuItem key={option} value={option}>
            {option}
          </MenuItem>
        ))}
      </Select>
    );
  }
};
