import React from 'react';
import { mapValues } from 'lodash-es';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import CircularProgress from '@mui/material/CircularProgress';
import Toolbar from '@mui/material/Toolbar';
import { useTranslation } from 'react-i18next';
import { Formik, Form } from 'formik';
import { ResultRow, ResultTable } from '../Components/ResultTable';
import {
  FieldComponent,
  FieldGroup,
  FieldGroupComponent,
  InputFieldGroup,
} from '../Components/FieldGroup';
import { getValueParser } from '../Components/InputField';

function mapFieldComponents<Result>(
  fieldGroups: FieldGroup[],
  mapper: (FieldComponent) => Result,
): Record<string, Result> {
  const fields: FieldGroupComponent[] = fieldGroups.flatMap(
    fieldGroup => fieldGroup.components,
  );
  return fields.reduce((acc, component) => {
    if (component.type !== 'label') {
      acc[component.id] = mapper(component);
    }
    return acc;
  }, {});
}

export type SearcherConfig = {
  getFieldGroups: () => FieldGroup[];
  getResultColumns: () => string[];
  generateResults: (
    formValues: Record<string, string>,
  ) => ResultRow[] | Promise<ResultRow[]>;
};

type Props = {
  config: SearcherConfig;
};

export function ConfiguableSearcher({
  config: { getFieldGroups, getResultColumns, generateResults },
}: Props) {
  const { t } = useTranslation();
  const [isSearching, setIsSearching] = React.useState(false);
  const [results, setResults] = React.useState([]);

  const fieldGroups = React.useMemo(getFieldGroups, [getFieldGroups]);
  const resultColumns = React.useMemo(getResultColumns, [getResultColumns]);

  const initialValues = React.useMemo(() => {
    return mapFieldComponents(fieldGroups, component =>
      component.type === 'checkbox' ? false : component.defaultValue,
    );
  }, [fieldGroups]);

  const valueParsers = React.useMemo(() => {
    return mapFieldComponents(fieldGroups, component =>
      getValueParser(component.type),
    );
  }, [fieldGroups]);

  const handleSubmit = React.useCallback(
    async values => {
      setIsSearching(true);
      const stringifiedValues = mapValues(values, (value, key) => {
        return valueParsers[key](String(value));
      });
      try {
        const newResults = await generateResults(stringifiedValues);
        setResults(newResults);
      } catch {
        setResults([]);
      } finally {
        setIsSearching(false);
      }
    },
    [generateResults],
  );

  return (
    <>
      <Toolbar />

      <Formik initialValues={initialValues} onSubmit={handleSubmit}>
        {() => {
          return (
            <Box
              component={Form}
              sx={{
                width: { sm: '75%' },
                maxWidth: '1000px',
                ml: 'auto',
                mr: 'auto',
                mb: '30px',
                display: 'flex',
                flexDirection: 'column',
              }}
            >
              {fieldGroups.map(fieldGroup => (
                <InputFieldGroup
                  key={fieldGroup.label}
                  fieldGroup={fieldGroup}
                />
              ))}

              <Button
                disabled={isSearching}
                type="submit"
                variant="contained"
                fullWidth
                sx={{
                  margin: '10px',
                  ml: 'auto',
                  mr: 'auto',
                  maxWidth: '300px',
                }}
              >
                {isSearching ? <CircularProgress size={24} /> : t('Search')}
              </Button>

              <ResultTable columns={resultColumns} results={results} />
            </Box>
          );
        }}
      </Formik>
    </>
  );
}
