import React from 'react';
import TranslateIcon from '@mui/icons-material/Translate';
import IconButton from '@mui/material/IconButton';
import MenuItem from '@mui/material/MenuItem';
import Menu from '@mui/material/Menu';

import { useTranslation } from 'react-i18next';

const options = [
  { name: 'English', code: 'en' },
  { name: 'French', code: 'fr' },
  { name: 'Japanese', code: 'ja' },
];

export const Translate = () => {
  const { t, i18n } = useTranslation();
  const [anchorEl, setAnchorEl] = React.useState(null);
  const open = Boolean(anchorEl);
  const handleClickListItem = event => {
    setAnchorEl(event.currentTarget);
  };

  const handleMenuItemClick = (event, option) => {
    setAnchorEl(null);
    i18n.changeLanguage(option);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };
  return (
    <React.Fragment>
      <IconButton
        size="medium"
        edge="end"
        color="inherit"
        aria-label="translate"
        sx={{ ml: 2 }}
        onClick={handleClickListItem}
      >
        <TranslateIcon />
      </IconButton>
      <Menu
        id="language-menu"
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
      >
        {options.map(option => (
          <MenuItem
            key={option.code}
            selected={option.code === i18n.resolvedLanguage}
            onClick={event => handleMenuItemClick(event, option.code)}
          >
            {option.name}
          </MenuItem>
        ))}
      </Menu>
    </React.Fragment>
  );
};
