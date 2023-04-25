import React from 'react';
import TableRow from '@mui/material/TableRow';
import TableCell from '@mui/material/TableCell';

const Result = ({ target, t }) => {
  return (
    <React.Fragment>
      <TableCell align="left">{target.advances}</TableCell>
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

export const Target = ({ state, t }) => {
  const { target } = state;

  return (
    <React.Fragment>
      <TableRow selected={true}>
        {target.is_set ? (
          <Result target={target} t={t} />
        ) : (
          <TableCell colSpan={6} align="left">
            {t('Click or tap a result to set as target')}
          </TableCell>
        )}
      </TableRow>
    </React.Fragment>
  );
};
