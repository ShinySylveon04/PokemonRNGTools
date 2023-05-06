import React from 'react';
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

export type ResultRow = string[];

type ResultRowsProps = {
  page: number;
  rowsPerPage: number;
  results: ResultRow[];
};

const ResultRows = ({ page, rowsPerPage, results }: ResultRowsProps) => {
  const { t } = useTranslation();

  const startIndex = page * rowsPerPage;
  const endIndex = startIndex + rowsPerPage;
  const viewableResults =
    rowsPerPage > 0 ? results.slice(startIndex, endIndex) : results;

  return (
    <>
      {viewableResults.map((resultRow, rowIndex) => {
        const resultIndex = startIndex + rowIndex;
        return (
          <TableRow
            key={resultIndex}
            sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
          >
            {resultRow.map((result, columnIndex) => {
              return (
                <TableCell key={`${resultIndex}-${columnIndex}`} align="left">
                  {t(result)}
                </TableCell>
              );
            })}
          </TableRow>
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
  const [rowsPerPage, setRowsPerPage] = React.useState(10);
  const [page, setPage] = React.useState(0);

  const handleChangePage = (event, newPage) => {
    setPage(newPage);
  };

  const handleChangeRowsPerPage = event => {
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
            <ResultRows
              page={page}
              rowsPerPage={rowsPerPage}
              results={results}
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
