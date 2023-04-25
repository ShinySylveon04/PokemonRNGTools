import React from 'react';
import TableRow from '@mui/material/TableRow';
import TableCell from '@mui/material/TableCell';

import { useTranslation } from 'react-i18next';

const Result = ({ target }) => {
  return (
    <React.Fragment>
      <TableCell align="left">{target.advance}</TableCell>
      <TableCell align="left" sx={{ whiteSpace: 'nowrap' }}>
        {`${target.ivs[0]} /
          ${target.ivs[1]} /
          ${target.ivs[2]} /
          ${target.ivs[3]} /
          ${target.ivs[4]} /
          ${target.ivs[5]}`}
      </TableCell>
      <TableCell align="left">
        {t(`hiddenpower.${target.hidden_power}`)}
      </TableCell>
      <TableCell align="left">{target.psv}</TableCell>
      <TableCell align="left">{target.pid.toString(16)}</TableCell>
    </React.Fragment>
  );
};

export const Target = ({ state }) => {
  const { t } = useTranslation();
  const { target } = state;

  return (
    <React.Fragment>
      <TableRow>
        {target.set ? (
          <Result target={target} />
        ) : (
          <TableCell colSpan={6} align="left">
            {t('Click or tap a result to set as target')}
          </TableCell>
        )}
      </TableRow>
    </React.Fragment>
  );
};
