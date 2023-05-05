import React from 'react';

import DialogTitle from '@mui/material/DialogTitle';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';

export const SaveInfoDialog = ({
  dialogOpen,
  handleDialogClose,
  handleSaveProfile,
}) => {
  const [profileName, setProfileName] = React.useState('No Profile');
  return (
    <Dialog open={dialogOpen} onClose={handleDialogClose}>
      <DialogTitle>Name Profile</DialogTitle>
      <DialogContent>
        <DialogContentText>Type in the name of the profile.</DialogContentText>
        <TextField
          autoFocus
          autoComplete="off"
          margin="dense"
          id="saveDataName"
          label="Profile Name"
          type="text"
          fullWidth
          variant="standard"
          onChange={event => setProfileName(event.target.value)}
        />
      </DialogContent>
      <DialogActions>
        <Button onClick={() => handleSaveProfile(profileName)}>Save</Button>
      </DialogActions>
    </Dialog>
  );
};
