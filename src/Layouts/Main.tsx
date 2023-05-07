import React from 'react';
import { Outlet } from 'react-router-dom';
import Box from '@mui/material/Box';
import { styled, useTheme } from '@mui/material/styles';
import useMediaQuery from '@mui/material/useMediaQuery';
import { useLocation } from 'react-router-dom';

import { AppBar } from '../Components/AppBar';
import { NavDrawer } from '../Components/NavDrawer';

const Main = styled('main')(({ theme }) => ({
  flexGrow: 1,
  padding: theme.spacing(3),
  width: '75%',
}));

export const MainLayout = () => {
  const theme = useTheme();
  const isLargerScreen = useMediaQuery(theme.breakpoints.up('md'));
  const [isopen, setOpen] = React.useState(false);

  // @ts-ignore
  window.gtag('event', 'page_view', { page_location: useLocation().pathname });

  React.useEffect(() => {
    if (isLargerScreen) {
      setOpen(true);
    }
  }, [isLargerScreen]);

  const handleDrawerClose = () => {
    setOpen(false);
  };
  const handleDrawerToggle = () => {
    setOpen(!isopen);
  };
  return (
    <React.Fragment>
      <Box sx={{ display: 'flex' }}>
        <AppBar
          handleDrawerToggle={handleDrawerToggle}
          isLargerScreen={isLargerScreen}
        />
        <NavDrawer
          isopen={isopen}
          isLargerScreen={isLargerScreen}
          handleDrawerClose={handleDrawerClose}
        />
        <Main>
          <Outlet />
        </Main>
      </Box>
    </React.Fragment>
  );
};
