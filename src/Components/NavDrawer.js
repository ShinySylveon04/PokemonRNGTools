import React from 'react';
import Box from '@mui/material/Box';
import Drawer from '@mui/material/Drawer';
import Toolbar from '@mui/material/Toolbar';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemText from '@mui/material/ListItemText';
import ListItemButton from '@mui/material/ListItemButton';
import { Link, useLocation } from 'react-router-dom';

const drawerWidth = 240;

const NavItems = ({ handleDrawerClose, location }) => (
  <Box sx={{ overflow: 'auto' }}>
    <List>
      <ListItem>
        <ListItemText primary="Sword & Shield" />
      </ListItem>
      <ListItemButton
        selected={location === '/swsh'}
        onClick={handleDrawerClose}
        component={Link}
        to="swsh"
        sx={{ pl: 4 }}
      >
        <ListItemText primary="Overworld" />
      </ListItemButton>
      <ListItem>
        <ListItemText primary="Brilliant Diamond & Shining Pearl" />
      </ListItem>
      <ListItemButton
        selected={location === '/bdsp'}
        onClick={handleDrawerClose}
        component={Link}
        to="bdsp"
        sx={{ pl: 4 }}
      >
        <ListItemText primary="Wild" />
      </ListItemButton>
      <ListItemButton
        selected={location === '/bdsp/stationary'}
        onClick={handleDrawerClose}
        component={Link}
        to="bdsp/stationary"
        sx={{ pl: 4 }}
      >
        <ListItemText primary="Stationary" />
      </ListItemButton>
      <ListItemButton
        selected={location === '/bdsp/underground'}
        onClick={handleDrawerClose}
        component={Link}
        to="bdsp/underground"
        sx={{ pl: 4 }}
      >
        <ListItemText primary="Underground" />
      </ListItemButton>
      <ListItemButton
        selected={location === '/bdsp/tid'}
        onClick={handleDrawerClose}
        component={Link}
        to="bdsp/tid"
        sx={{ pl: 4 }}
      >
        <ListItemText primary="TID" />
      </ListItemButton>
      <ListItemButton
        selected={location === '/bdsp/roamers'}
        onClick={handleDrawerClose}
        component={Link}
        to="bdsp/roamers"
        sx={{ pl: 4 }}
      >
        <ListItemText primary="Roamers" />
      </ListItemButton>
    </List>
  </Box>
);

export const NavDrawer = ({ isopen, isLargerScreen, handleDrawerClose }) => {
  const location = useLocation().pathname;
  return (
    <Box
      component="nav"
      sx={{ width: { sm: drawerWidth }, flexShrink: { sm: 0 } }}
      aria-label="nav-drawer"
    >
      <Drawer
        open={isopen}
        onClose={handleDrawerClose}
        variant={isLargerScreen ? 'permanent' : 'temporary'}
        sx={{
          width: drawerWidth,
          flexShrink: 0,
          [`& .MuiDrawer-paper`]: {
            width: drawerWidth,
            boxSizing: 'border-box',
          },
        }}
      >
        <Toolbar />
        <NavItems handleDrawerClose={handleDrawerClose} location={location} />
      </Drawer>
    </Box>
  );
};
