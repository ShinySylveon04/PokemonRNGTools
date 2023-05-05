import React from 'react';

import Grid from '@mui/material/Grid';
import Button from '@mui/material/Button';
import SaveAltIcon from '@mui/icons-material/SaveAlt';

export const SaveProfile = ({ handleDialogOpen }) => {
  return (
    <Grid
      container
      item
      sm={3}
      xs={12}
      sx={{ justifyContent: { xs: 'center', md: 'flex-start' } }}
    >
      <Button variant="contained" onClick={handleDialogOpen}>
        Save Info <SaveAltIcon />
      </Button>
    </Grid>
  );
};
