import React from 'react';
import TableCell from '@mui/material/TableCell';
import TableRow from '@mui/material/TableRow';
import { useTranslation } from 'react-i18next';

type Props = {
  colSpan?: number;
};

export const NoResults = ({ colSpan = 3 }: Props) => {
  const { t } = useTranslation();
  return (
    <TableRow sx={{ '&:last-child td, &:last-child th': { border: 0 } }}>
      <TableCell align="center" colSpan={colSpan}>
        {t(`No Results Found`)}
      </TableCell>
    </TableRow>
  );
};
