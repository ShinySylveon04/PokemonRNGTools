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

import { natures } from '../natures';
import { NoResults } from '../Components/NoResults';

const shinyType = value => {
  switch (value) {
    case 1:
      return 'Star';
    case 2:
      return 'Square';
    default:
      return 'None';
  }
};

const abilityType = value => {
  switch (value) {
    case 1:
      return 0;
    default:
      return 1;
  }
};

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
        <TableCell align="left">{shinyType(result.shiny_value)}</TableCell>
        <TableCell align="left">{natures[result.nature]}</TableCell>
        <TableCell align="left">{abilityType(result.ability)}</TableCell>
        <TableCell align="left">{result.state0}</TableCell>
        <TableCell align="left">{result.state1}</TableCell>
        <TableCell align="left">{result.ec}</TableCell>
        <TableCell align="left">{result.pid}</TableCell>
      </TableRow>
    ));
  }
};

export const Results = ({ results }) => {
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
              <TableCell>Advances</TableCell>
              <TableCell align="left">Shiny</TableCell>
              <TableCell align="left">Nature</TableCell>
              <TableCell align="left">Ability</TableCell>
              <TableCell align="left">State 0</TableCell>
              <TableCell align="left">State 1</TableCell>
              <TableCell align="left">EC</TableCell>
              <TableCell align="left">PID</TableCell>
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
