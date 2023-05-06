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

type ResultRow = string[];

type ResultRowsProps = {
  results: ResultRow[];
};

const ResultRows = ({ results }: ResultRowsProps) => {
  const { t } = useTranslation();

  return (
    <>
      {results.map(resultRow => {
        return (
          <TableRow sx={{ '&:last-child td, &:last-child th': { border: 0 } }}>
            {resultRow.map(result => {
              return <TableCell align="left">{t(result)}</TableCell>;
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
  const [selected, setSelected] = React.useState([]);

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
                <TableCell>{t(column)}</TableCell>
              ))}
            </TableRow>
          </TableHead>
          <TableBody>
            <ResultRows
              results={
                rowsPerPage > 0
                  ? results.slice(
                      page * rowsPerPage,
                      page * rowsPerPage + rowsPerPage,
                    )
                  : results
              }
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
