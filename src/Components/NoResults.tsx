import React from 'react';
import TableCell from '@mui/material/TableCell';
import TableRow from '@mui/material/TableRow';

export const NoResults = ({ t }) => {
  return (
    <TableRow sx={{ '&:last-child td, &:last-child th': { border: 0 } }}>
      <TableCell align="center" colSpan={3}>
        {t(`No Results Found`)}
      </TableCell>
    </TableRow>
  );
};
