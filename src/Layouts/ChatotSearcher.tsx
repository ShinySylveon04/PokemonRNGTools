import React from 'react';
import { ResultRow } from '../Components/ResultTable';
import { SerializedValue } from '../Components/InputField';
import { useWasm } from '../wasm/context';
import type {
  GetResultColumnFunctionName,
  GetFieldGroupFunctionName,
} from '../wasm/types';
import { ConfiguableSearcher } from './ConfiguableSearcher';

export type SearcherConfig = {
  getFieldGroupsName: GetFieldGroupFunctionName;
  getResultColumnsName: GetResultColumnFunctionName;
  generateResults: (
    formValues: Record<string, SerializedValue>,
  ) => ResultRow[] | Promise<ResultRow[]>;
};

type Props = {
  config: SearcherConfig;
};

export function ChatotSearcher({
  config: { getFieldGroupsName, getResultColumnsName, generateResults },
}: Props) {
  const { isLoading, initializedWasm } = useWasm();

  const { fieldGroups, resultColumns } = React.useMemo(() => {
    if (isLoading) {
      return { fieldGroups: [], resultColumns: [] };
    }
    return {
      fieldGroups: initializedWasm[getFieldGroupsName](),
      resultColumns: initializedWasm[getResultColumnsName](),
    };
  }, [isLoading, initializedWasm, getFieldGroupsName, getResultColumnsName]);

  return (
    <ConfiguableSearcher
      loading={isLoading}
      fieldGroups={fieldGroups}
      resultColumns={resultColumns}
      generateResults={generateResults}
    />
  );
}
