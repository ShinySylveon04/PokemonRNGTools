import React from 'react';
import { ResultRow } from '../Components/ResultTable';
import { SerializedValue } from '../Components/InputField';
import { ConfiguableSearcher } from './ConfiguableSearcher';
import { FieldGroup } from '../Components/FieldGroup';

type Props = {
  getFieldGroups: () => FieldGroup[];
  getResultColumns: () => string[];
  generateResults: (
    formValues: Record<string, SerializedValue>,
  ) => ResultRow[] | Promise<ResultRow[]>;
};

export function PluginSearcher({
  getFieldGroups,
  getResultColumns,
  generateResults,
}: Props) {
  const { fieldGroups, resultColumns } = React.useMemo(() => {
    return {
      fieldGroups: getFieldGroups(),
      resultColumns: getResultColumns(),
    };
  }, [getFieldGroups, getResultColumns]);

  return (
    <ConfiguableSearcher
      fieldGroups={fieldGroups}
      resultColumns={resultColumns}
      generateResults={generateResults}
    />
  );
}
