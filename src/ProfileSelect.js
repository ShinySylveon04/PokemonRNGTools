import React from 'react';

import Grid from '@mui/material/Grid';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';

export const ProfileSelect = ({ saveProfile }) => {
  const name = saveProfile ? saveProfile.name : 'No Profile';
  return (
    <Grid item sm={3} md={3} xs={12}>
      <FormControl fullWidth>
        <InputLabel id="saved-info-label">Current Info</InputLabel>
        <Select
          labelId="saved-info-label"
          id="saved-info"
          value={name}
          label="Current Info"
        >
          <MenuItem value={name}>{name}</MenuItem>
        </Select>
      </FormControl>
    </Grid>
  );
};
