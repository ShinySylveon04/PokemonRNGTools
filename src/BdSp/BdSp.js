import React from 'react';

import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';

import { Wild } from './Wild/Wild';
import { Stationary } from './Stationary/Stationary';

function a11yProps(index) {
  return {
    id: `bdsp-tab-${index}`,
    'aria-controls': `bdsp-${index}`,
  };
}

function TabPanel(props) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`bdsp-tabpanel-${index}`}
      aria-labelledby={`bdsp-tab-${index}`}
      {...other}
    >
      {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
    </div>
  );
}

export const BdSp = () => {
  const [value, setValue] = React.useState(0);

  const handleChange = (event, newValue) => {
    setValue(newValue);
  };
  return (
    <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
      <Typography variant="h3" gutterBottom align="center">
        Brilliant Diamond & Shining Pearl RNG
      </Typography>
      <Tabs
        centered
        value={value}
        onChange={handleChange}
        aria-label="bdsp tabs"
      >
        <Tab label="Wild" {...a11yProps(0)} />
        <Tab label="Stationary" {...a11yProps(1)} />
      </Tabs>
      <TabPanel value={value} index={0}>
        <Wild />
      </TabPanel>
      <TabPanel value={value} index={1}>
        <Stationary />
      </TabPanel>
    </Box>
  );
};
