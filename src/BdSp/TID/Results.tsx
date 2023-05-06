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
import { NoResults } from '../../Components/NoResults';

const ShowResults = ({ results, t }) => {
  if (results.length === 0) {
    return <NoResults />;
  } else {
    return results.map((result, index) => (
      <TableRow
        key={index}
        sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
      >
        <TableCell align="left">{result.advances}</TableCell>
        <TableCell align="left">{result.g8tid}</TableCell>
        <TableCell align="left">{result.tid}</TableCell>
        <TableCell align="left">{result.sid}</TableCell>
        <TableCell align="left">{result.tsv}</TableCell>
      </TableRow>
    ));
  }
};

export const Results = ({ results, state }) => {
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

  // Avoid a layout jump when reaching the last page with empty rows.
  const emptyRows =
    page > 0 ? Math.max(0, (1 + page) * rowsPerPage - results.length) : 0;

  return (
    <React.Fragment>
      <TableContainer component={Paper}>
        <Table sx={{ minWidth: 650 }} aria-label="results table">
          <TableHead>
            <TableRow>
              <TableCell>{t('Advances')}</TableCell>
              <TableCell>{t('Gen 8 TID')}</TableCell>
              <TableCell>{t('TID')}</TableCell>
              <TableCell>{t('SID')}</TableCell>
              <TableCell>{t('TSV')}</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            <ShowResults
              t={t}
              results={
                rowsPerPage > 0
                  ? results.slice(
                      page * rowsPerPage,
                      page * rowsPerPage + rowsPerPage,
                    )
                  : results
              }
            />
            {emptyRows > 0 && (
              <TableRow style={{ height: 53 * emptyRows }}>
                <TableCell colSpan={6} />
              </TableRow>
            )}
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
