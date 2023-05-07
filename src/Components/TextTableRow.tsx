import React from 'react';
import TableCell from '@mui/material/TableCell';
import TableRow from '@mui/material/TableRow';

type Props = {
  colSpan?: number;
  selected?: boolean;
};

export const TextTableRow = ({
  colSpan,
  selected,
  children,
}: React.PropsWithChildren<Props>) => {
  return (
    <TableRow
      sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
      selected={selected}
    >
      <TableCell align="center" colSpan={colSpan}>
        {children}
      </TableCell>
    </TableRow>
  );
};
