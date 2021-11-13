import React from 'react';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';

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

export const Results = ({ results }) => {
  return (
    <TableContainer component={Paper}>
      <Table sx={{ minWidth: 650 }} size="small" aria-label="simple table">
        <TableHead>
          <TableRow>
            <TableCell>Advances</TableCell>
            <TableCell align="left">Shiny</TableCell>
            <TableCell align="left">State 0</TableCell>
            <TableCell align="left">State 1</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          <TableRow sx={{ '&:last-child td, &:last-child th': { border: 0 } }}>
            <TableCell align="left">{results.advances}</TableCell>
            <TableCell align="left">{shinyType(results.shiny_value)}</TableCell>
            <TableCell align="left">
              {BigInt(results.state0, 16).toString(16)}
            </TableCell>
            <TableCell align="left">
              {BigInt(results.state1, 16).toString(16)}
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </TableContainer>
  );
};
