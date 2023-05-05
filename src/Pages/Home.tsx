import React from 'react';

import Toolbar from '@mui/material/Toolbar';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';
import MuiLink from '@mui/material/Link';
import Button from '@mui/material/Button';
import Grid from '@mui/material/Grid';
import Card from '@mui/material/Card';
import CardHeader from '@mui/material/CardHeader';
import CardContent from '@mui/material/CardContent';

import { Link } from 'react-router-dom';

import explorerKit from '../Resources/explorer_kit.png';
import honey from '../Resources/honey.png';
import dialga from '../Resources/Dialga.png';

export const Home = () => {
  return (
    <Box
      sx={{
        width: { sm: '75%' },
        maxWidth: '700px',
        ml: 'auto',
        mr: 'auto',
        mb: '30px',
        display: 'flex',
        flexDirection: 'column',
      }}
    >
      <Toolbar />
      <Typography variant="h3" gutterBottom>
        Welcome to Chatot
      </Typography>
      <Typography variant="h4" gutterBottom>
        What is Chatot?
      </Typography>
      <Typography variant="body1" paragraph>
        Chatot is a web based app with tools for use with Pokemon games to RNG
        the Pokemon you want. For more information on what RNG is and how to do
        it, visit the main site at{' '}
        <MuiLink href="https://www.pokemonrng.com">
          https://www.pokemonrng.com
        </MuiLink>
        .
      </Typography>
      <Typography variant="h4" gutterBottom>
        Chatot Tools
      </Typography>
      <Grid container spacing={2}>
        <Grid item xs={12} sm={6}>
          <Card variant="outlined">
            <CardHeader
              avatar={<img src={honey} alt="Honey" width={50} />}
              title="Brilliant Diamond & Shining Pearl"
              subheader="Wild"
            />
            <CardContent>
              <Typography variant="body2" gutterBottom>
                RNG Pokemon encountered in the wild
              </Typography>
              <Button
                color="primary"
                variant="contained"
                component={Link}
                to="bdsp"
                fullWidth
              >
                Link
              </Button>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} sm={6}>
          <Card variant="outlined">
            <CardHeader
              avatar={<img src={dialga} alt="Dialga" width={50} />}
              title="Brilliant Diamond & Shining Pearl"
              subheader="Static"
            />
            <CardContent>
              <Typography variant="body2" gutterBottom>
                RNG static Pokemon, such as your starter or Dialga/Palkia
              </Typography>
              <Button
                color="primary"
                variant="contained"
                component={Link}
                to="bdsp/static"
                fullWidth
              >
                Link
              </Button>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} sm={6}>
          <Card variant="outlined">
            <CardHeader
              avatar={<img src={explorerKit} alt="Explorer Kit" width={50} />}
              title="Brilliant Diamond & Shining Pearl"
              subheader="Underground"
            />
            <CardContent>
              <Typography variant="body2" gutterBottom>
                RNG Pokemon in the Underground
              </Typography>
              <Button
                color="primary"
                variant="contained"
                component={Link}
                to="bdsp/underground"
                fullWidth
              >
                Link
              </Button>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  );
};
