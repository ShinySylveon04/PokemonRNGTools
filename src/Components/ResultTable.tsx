import React from 'react';
import { noop } from 'lodash-es';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';
import TablePagination from '@mui/material/TablePagination';

import { useTranslation } from 'react-i18next';
import { NoResults } from '../Components/NoResults';
import { TextTableRow } from '../Components/TextTableRow';

export type ResultRow = string[];

type ResultRowProps = {
  resultRow: ResultRow;
  selected?: boolean;
  onClick?: (row: ResultRow) => void;
};

const ResultRow = ({ resultRow, selected, onClick = noop }: ResultRowProps) => {
  const { t } = useTranslation();
  return (
    <TableRow
      sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
      selected={selected}
      onClick={() => onClick(resultRow)}
    >
      {resultRow.map((result, columnIndex) => {
        return (
          <TableCell key={columnIndex} align="left">
            {t(result)}
          </TableCell>
        );
      })}
    </TableRow>
  );
};

type ResultRowsProps = {
  page: number;
  rowsPerPage: number;
  results: ResultRow[];
  onRowClick?: (row: ResultRow) => void;
};

const ResultRows = ({
  page,
  rowsPerPage,
  results,
  onRowClick,
}: ResultRowsProps) => {
  const startIndex = page * rowsPerPage;
  const endIndex = startIndex + rowsPerPage;
  const viewableResults =
    rowsPerPage > 0 ? results.slice(startIndex, endIndex) : results;

  return (
    <>
      {viewableResults.map((resultRow, rowIndex) => {
        const resultIndex = startIndex + rowIndex;
        return (
          <ResultRow
            key={resultIndex}
            resultRow={resultRow}
            onClick={onRowClick}
          />
        );
      })}
    </>
  );
};

type Props = {
  columns: string[];
  results: ResultRow[];
};

export const ResultTable = ({ columns, results }: Props) => {
  const { t } = useTranslation();
  const [targetRow, setTargetRow] = React.useState<string[] | null>(null);
  const [rowsPerPage, setRowsPerPage] = React.useState(10);
  const [page, setPage] = React.useState(0);

  const handleChangePage = (
    event: React.MouseEvent | null,
    newPage: number,
  ) => {
    setPage(newPage);
  };

  const handleChangeRowsPerPage = (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    setRowsPerPage(parseInt(event.target.value, 10));
    setPage(0);
  };

  return (
    <React.Fragment>
      <TableContainer component={Paper}>
        <Table sx={{ minWidth: 650 }} aria-label="results table">
          <TableHead>
            <TableRow>
              {columns.map(column => (
                <TableCell key={column}>{t(column)}</TableCell>
              ))}
            </TableRow>
          </TableHead>
          <TableBody>
            {targetRow != null && <ResultRow resultRow={targetRow} selected />}
            {targetRow == null && (
              <TextTableRow colSpan={columns.length} selected>
                {t('Click or tap a result to set as target')}
              </TextTableRow>
            )}
            <ResultRows
              page={page}
              rowsPerPage={rowsPerPage}
              results={results}
              onRowClick={setTargetRow}
            />
            {results.length === 0 && <NoResults colSpan={columns.length} />}
          </TableBody>
        </Table>
        <TablePagination
          sx={{ overflow: 'initial' }}
          rowsPerPageOptions={[10, 25, 50, 100]}
          component="div"
          count={results.length}
          rowsPerPage={rowsPerPage}
          page={page}
          onPageChange={handleChangePage}
          onRowsPerPageChange={handleChangeRowsPerPage}
        />
      </TableContainer>
    </React.Fragment>
  );
};
