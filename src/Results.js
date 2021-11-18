import React from 'react';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';

import { natures } from './natures';

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

const ShowResults = ({ results }) => {
  return results.map((result, index) => (
    <TableRow
      key={index}
      sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
    >
      <TableCell align="left">{result.advances}</TableCell>
      <TableCell align="left">{shinyType(result.shiny_value)}</TableCell>
      <TableCell align="left">
        {BigInt(result.state0, 16).toString(16)}
      </TableCell>
      <TableCell align="left">
        {BigInt(result.state1, 16).toString(16)}
      </TableCell>
      <TableCell align="left">{BigInt(result.ec, 16).toString(16)}</TableCell>
      <TableCell align="left">{BigInt(result.pid, 16).toString(16)}</TableCell>
      <TableCell align="left">{natures[result.nature]}</TableCell>
    </TableRow>
  ));
};

export const Results = ({ results }) => {
  return (
    <TableContainer component={Paper}>
      <Table sx={{ minWidth: 650 }} aria-label="results table">
        <TableHead>
          <TableRow>
            <TableCell>Advances</TableCell>
            <TableCell align="left">Shiny</TableCell>
            <TableCell align="left">State 0</TableCell>
            <TableCell align="left">State 1</TableCell>
            <TableCell align="left">EC</TableCell>
            <TableCell align="left">PID</TableCell>
            <TableCell align="left">Nature</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          <ShowResults results={results} />
        </TableBody>
      </Table>
    </TableContainer>
  );
};
