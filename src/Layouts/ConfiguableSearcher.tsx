import React from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import CircularProgress from '@mui/material/CircularProgress';
import Toolbar from '@mui/material/Toolbar';
import { useTranslation } from 'react-i18next';
import { ResultTable } from '../Components/ResultTable';
import { FieldGroup, InputFieldGroup } from '../Components/FieldGroup';

type Props = {
  fieldGroups: FieldGroup[];
  resultColumns: string[];
  generateResults: () => string[][];
};

export function ConfiguableSearcher({
  fieldGroups,
  resultColumns,
  generateResults,
}: Props) {
  const { t } = useTranslation();
  const [isSearching, setIsSearching] = React.useState(false);
  const [results, setResults] = React.useState([]);

  const handleSubmit = React.useCallback(() => {
    setIsSearching(true);
    setResults(generateResults());
    setIsSearching(false);
  }, [generateResults]);

  return (
    <Box
      component="form"
      autoComplete="off"
      onSubmit={handleSubmit}
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
      <Toolbar />

      {fieldGroups.map(fieldGroup => (
        <InputFieldGroup key={fieldGroup.label} fieldGroup={fieldGroup} />
      ))}

      <Button
        disabled={isSearching}
        type="submit"
        variant="contained"
        fullWidth
        sx={{ margin: '10px', ml: 'auto', mr: 'auto', maxWidth: '300px' }}
      >
        {isSearching ? <CircularProgress size={24} /> : t('Search')}
      </Button>

      <ResultTable columns={resultColumns} results={results} />
    </Box>
  );
}
