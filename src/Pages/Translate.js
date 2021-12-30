import React from 'react';

import Toolbar from '@mui/material/Toolbar';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';
import MuiLink from '@mui/material/Link';
import Button from '@mui/material/Button';

export const Translate = () => {
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
        Translations
      </Typography>
      <Typography variant="h4" gutterBottom>
        Help Translate Chatot
      </Typography>
      <Typography varaint="body1" paragraph>
        If you would like to help translate Chatot into other languages, you can
        join the Pokemon RNG Discord and let me (Shiny_Sylveon) know that you
        are interested in helping translate.
      </Typography>
      <Button
        color="primary"
        variant="contained"
        component={MuiLink}
        href="https://www.discord.gg/d8JuAvg"
        sx={{ maxWidth: '250px', mb: '10px' }}
      >
        Pokemon RNG Discord
      </Button>
      <Typography variant="body1" paragraph>
        The translation json files can be found on the Github repository{' '}
        <MuiLink href="https://github.com/ShinySylveon04/PokemonRNGTools/tree/main/src/Translations">
          here
        </MuiLink>{' '}
        and pull requests to update or add translations are welcomed.
      </Typography>
    </Box>
  );
};
