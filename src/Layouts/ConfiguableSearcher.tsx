import React from 'react';
import { mapValues } from 'lodash-es';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import CircularProgress from '@mui/material/CircularProgress';
import Skeleton from '@mui/material/Skeleton';
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
import {
  DeserializedValue,
  SerializedValue,
  deserialize,
  serialize,
} from '../Components/InputField';

function mapFieldComponents<Result>(
  fieldGroups: FieldGroup[],
  mapper: (component: FieldComponent) => Result,
): Record<string, Result> {
  const fields: FieldGroupComponent[] = fieldGroups.flatMap(
    fieldGroup => fieldGroup.components,
  );
  return fields.reduce((acc: Record<string, Result>, component) => {
    if (component.type !== 'label') {
      acc[component.id] = mapper(component);
    }
    return acc;
  }, {});
}

type Props = {
  loading?: boolean;
  fieldGroups: FieldGroup[];
  resultColumns: string[];
  generateResults: (
    formValues: Record<string, SerializedValue>,
  ) => ResultRow[] | Promise<ResultRow[]>;
};

export function ConfiguableSearcher({
  loading,
  fieldGroups,
  resultColumns,
  generateResults,
}: Props) {
  const { t } = useTranslation();
  const [isSearching, setIsSearching] = React.useState(false);
  const [results, setResults] = React.useState<ResultRow[]>([]);

  const serializers = React.useMemo(() => {
    return mapFieldComponents(
      fieldGroups ?? [],
      component => (value: DeserializedValue) =>
        serialize({ type: component.type, value }),
    );
  }, [fieldGroups]);

  const initialValues = React.useMemo(() => {
    return mapFieldComponents(fieldGroups ?? [], component => {
      return deserialize({
        type: component.type,
        value: component.defaultValue || null,
      });
    });
  }, [fieldGroups]);

  const handleSubmit = React.useCallback(
    async values => {
      setIsSearching(true);
      const serialized = mapValues(values, (value, key) => {
        return serializers[key](value);
      });
      try {
        const newResults = await generateResults(serialized);
        setResults(newResults);
      } catch {
        setResults([]);
      } finally {
        setIsSearching(false);
      }
    },
    [generateResults, serializers],
  );

  return (
    <>
      <Toolbar />

      <Formik
        enableReinitialize
        initialValues={initialValues}
        onSubmit={handleSubmit}
      >
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
              {loading ? (
                <>
                  <InputFieldGroup loading />
                  <InputFieldGroup loading />
                </>
              ) : (
                fieldGroups.map(fieldGroup => (
                  <InputFieldGroup
                    key={fieldGroup.label}
                    fieldGroup={fieldGroup}
                  />
                ))
              )}

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

              {loading ? (
                <Skeleton variant="rectangular" height={200} />
              ) : (
                <ResultTable columns={resultColumns} results={results} />
              )}
            </Box>
          );
        }}
      </Formik>
    </>
  );
}
