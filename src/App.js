import React from 'react';
import { calculate_pokemon } from '../wasm/Cargo.toml';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Container from '@mui/material/Container';

import { TrainerInfo } from './TrainerInfo';
import { RNGInfo } from './RNGInfo';
import { Filters } from './Filters';
import { Results } from './Results';

export function App() {
  const [state, setState] = React.useState({
    state0: BigInt('0xe1e16bc81e378a0b'),
    state1: BigInt('0xa79a405a9d7f5849'),
    tid: 0,
    sid: 0,
    shiny: 0,
    encounter: 0,
    shinyCharm: false,
    version: 'Sword',
    badgeCount: 0,
  });
  React.useEffect(() => {
    const isBrowser = typeof window !== 'undefined';
    const storageProfile = isBrowser
      ? JSON.parse(localStorage.getItem('Profiles'))
      : null;
    storageProfile &&
      setState({
        ...state,
        tid: storageProfile.tid,
        sid: storageProfile.sid,
        shinyCharm: storageProfile.shinyCharm,
        version: storageProfile.version,
        badgeCount: storageProfile.badgeCount,
      });
    storageProfile && setProfile(storageProfile);
  }, []);
  const [saveProfile, setProfile] = React.useState({
    name: 'No Profile',
    tid: 0,
    sid: 0,
    shinyCharm: false,
    version: 'Sword',
    badgeCount: 0,
  });
  const [results, setResults] = React.useState({
    advances: 0,
    shiny_value: 0,
    state0: 0,
    state1: 0,
    ec: 0,
    pid: 0,
  });
  const { state0, state1, tid, sid, shiny, encounter, shinyCharm } = state;
  const findResults = event => {
    event.preventDefault();
    const shiny_result = calculate_pokemon(
      state0,
      state1,
      tid,
      sid,
      shiny,
      encounter,
      shinyCharm,
    );
    setResults(shiny_result);
  };

  return (
    <Container>
      <Box
        component="form"
        autoComplete="off"
        onSubmit={findResults}
        sx={{
          width: { sm: '75%' },
          maxWidth: '800px',
          ml: 'auto',
          mr: 'auto',
          mb: '30px',
          display: 'flex',
          flexDirection: 'column',
        }}
      >
        <Typography variant="h3" gutterBottom align="center">
          Sword & Shield RNG
        </Typography>
        <TrainerInfo
          setState={setState}
          state={state}
          saveProfile={saveProfile}
          setProfile={setProfile}
        />
        <RNGInfo setState={setState} state={state} />
        <Filters setState={setState} state={state} />
        <Button
          type="submit"
          variant="contained"
          fullWidth
          sx={{ margin: '10px', ml: 'auto', mr: 'auto', maxWidth: '300px' }}
        >
          Search
        </Button>
        <Results results={results} />
      </Box>
    </Container>
  );
}
