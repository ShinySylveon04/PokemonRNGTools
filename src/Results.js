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
          <TableRow sx={{ '&:last-child td, &:last-child th': { border: 0 } }}>
            <TableCell align="left">{results.advances}</TableCell>
            <TableCell align="left">{shinyType(results.shiny_value)}</TableCell>
            <TableCell align="left">
              {BigInt(results.state0, 16).toString(16)}
            </TableCell>
            <TableCell align="left">
              {BigInt(results.state1, 16).toString(16)}
            </TableCell>
            <TableCell align="left">
              {BigInt(results.ec, 16).toString(16)}
            </TableCell>
            <TableCell align="left">
              {BigInt(results.pid, 16).toString(16)}
            </TableCell>
            <TableCell align="left">{natures[results.nature]}</TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </TableContainer>
  );
};
