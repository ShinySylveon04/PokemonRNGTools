import React from 'react';
import {
  isArray as _isArray,
  isNumber as _isNumber,
  isString as _isString,
} from 'lodash-es';
import TextField from '@mui/material/TextField';
import Checkbox from '@mui/material/Checkbox';
import FormControlLabel from '@mui/material/FormControlLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import Select from '@mui/material/Select';
import ListItemText from '@mui/material/ListItemText';
import { useTranslation } from 'react-i18next';
import { useFormikContext } from 'formik';

const ANY = 'Any';
const HEX_U32_REGEX = '[0-9A-Fa-f]{1,8}';
const HEX_U64_REGEX = '[0-9A-Fa-f]{1,16}';

type SharedConfig = {
  id: string;
  label: string;
  // Normally each field would have a specific type,
  // but we should be defensive against bad external configs.
  // This will also make types easier on the wasm side.
  defaultValue?: string;
  required?: boolean;
};

type SelectOption = {
  label: string;
  value: string;
};

export type FieldConfig = SharedConfig &
  (
    | {
        type: 'select' | 'optional_select' | 'multiselect';
        options: SelectOption[];
        minValue?: never;
        maxValue?: never;
      }
    | {
        type: 'number';
        options?: never;
        minValue?: number;
        maxValue?: number;
      }
    | {
        type: 'hex_u64' | 'hex_number' | 'text' | 'checkbox';
        options?: never;
        minValue?: never;
        maxValue?: never;
      }
  );

type Props = FieldConfig;

export const InputField = ({
  id,
  type,
  label: nonTranslatedLabel,
  options,
  required = true,
  minValue,
  maxValue,
}: Props) => {
  const { t } = useTranslation();
  const label = t(nonTranslatedLabel);
  const formik = useFormikContext<Record<string, unknown>>();

  const inputProps = React.useMemo(() => {
    if (type === 'number') {
      return { min: minValue, max: maxValue };
    }

    if (type === 'hex_number') {
      return { pattern: HEX_U32_REGEX };
    }

    if (type === 'hex_u64') {
      return { pattern: HEX_U64_REGEX };
    }

    return {};
  }, [type, minValue, maxValue]);

  // Ensures all inputs are controlled inputs
  const value = formik.values[id] ?? '';
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
        required={required}
        id={id}
        label={label}
        variant="outlined"
        type="number"
        autoComplete="off"
        onChange={formik.handleChange}
        value={value}
        error={error}
        helperText={helperText}
        inputProps={inputProps}
      />
    );
  }

  if (type === 'text') {
    return (
      <TextField
        fullWidth
        required={required}
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

  if (type === 'hex_number' || type === 'hex_u64') {
    return (
      <TextField
        fullWidth
        required={required}
        id={id}
        label={label}
        variant="outlined"
        autoComplete="off"
        onChange={formik.handleChange}
        value={value}
        error={error}
        helperText={helperText}
        inputProps={inputProps}
      />
    );
  }

  if (type === 'select' || type === 'optional_select') {
    return (
      <TextField
        select
        fullWidth
        required={required}
        id={id}
        name={id}
        label={label}
        autoComplete="off"
        onChange={formik.handleChange}
        value={value}
        error={error}
      >
        {type === 'optional_select' && (
          <MenuItem key={t(ANY)} value={ANY}>
            {t(ANY)}
          </MenuItem>
        )}
        {options.map(option => (
          <MenuItem key={option.label} value={option.value}>
            {t(option.label)}
          </MenuItem>
        ))}
      </TextField>
    );
  }

  if (type === 'multiselect') {
    const safeValue = isArray(value) ? value : [];
    return (
      <FormControl fullWidth>
        <InputLabel id={`${id}-label`} shrink>
          {label}
        </InputLabel>
        <Select
          multiple
          fullWidth
          displayEmpty
          notched
          required={required}
          id={id}
          name={id}
          label={label}
          autoComplete="off"
          onChange={formik.handleChange}
          value={safeValue}
          error={error}
          labelId={`${id}-label`}
          renderValue={selectedValues => {
            if (selectedValues.length === 0) {
              return t(ANY);
            }

            return selectedValues
              .map(selectedValue => {
                const option = options.find(
                  option => option.value == selectedValue,
                );
                return option == null ? null : t(option.label);
              })
              .join(', ');
          }}
        >
          {options.map(option => (
            <MenuItem key={option.label} value={option.value}>
              <Checkbox checked={safeValue.includes(option.value)} />
              <ListItemText primary={t(option.label)} />
            </MenuItem>
          ))}
        </Select>
      </FormControl>
    );
  }

  return null;
};

export type SerializedValue = string | boolean | number | string[] | null;
export type DeserializedValue =
  | string
  | boolean
  | number
  | string[]
  | typeof ANY;
type Deserializer = (value: SerializedValue) => DeserializedValue;

const DESERIALIZERS: Record<FieldConfig['type'], Deserializer> = {
  // JS can only handle up to 0x1fffffffffffff
  // and wasm can't natively handle bigint.
  // Converting String -> u64 will happen in wasm.
  hex_u64: (value: SerializedValue): string => {
    const stringified = String(value);
    const isValid = new RegExp(HEX_U64_REGEX).test(stringified);
    return isValid ? stringified : '0';
  },
  hex_number: (value: SerializedValue): number => {
    const stringified = String(value);
    const isValid = new RegExp(HEX_U32_REGEX).test(stringified);
    return isValid ? parseInt(stringified, 16) : 0;
  },
  number: (value: SerializedValue): number => {
    const num = parseInt(String(value), 10);
    return isNaN(num) ? 0 : num;
  },
  text: (value: SerializedValue): string => {
    return String(value);
  },
  select: (value: SerializedValue): string => {
    return String(value);
  },
  optional_select: (value: SerializedValue): string => {
    return value == null ? ANY : String(value);
  },
  multiselect: (value: SerializedValue): string[] => {
    return isArray(value) ? value : [];
  },
  checkbox: (value: SerializedValue): boolean => {
    return String(value) === 'true';
  },
};

const getDeserializer = (type: FieldConfig['type']): Deserializer => {
  const deserializer = DESERIALIZERS[type];
  return deserializer == null ? DESERIALIZERS.text : deserializer;
};

export const deserialize = ({
  type,
  value,
}: {
  type: FieldConfig['type'];
  value: SerializedValue;
}): DeserializedValue => {
  if (type === 'optional_select' && value == null) {
    return ANY;
  }

  if (type === 'multiselect' && value == null) {
    return [];
  }

  if (value == null) {
    return '';
  }

  const deserializer = getDeserializer(type);
  return deserializer(value);
};

export const serialize = ({
  type,
  value,
}: {
  type: FieldConfig['type'];
  value: DeserializedValue;
}): SerializedValue => {
  if (type === 'optional_select' && value === ANY) {
    return null;
  }

  const deserializer = getDeserializer(type);
  return deserializer(value);
};

export const isArray = (value: unknown): value is string[] => {
  if (_isArray(value)) {
    return value.find(item => !isString(item)) == null;
  }

  return false;
};

export const isNumber = (value: unknown): value is number => {
  return _isNumber(value);
};

export const isString = (value: unknown): value is string => {
  return _isString(value);
};
