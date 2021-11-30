import React from 'react';
import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid';
import TextField from '@mui/material/TextField';

export const IVFilters = ({ state, setState }) => {
  return (
    <React.Fragment>
      <Grid item sm={12} md={12} xs={12}>
        <Typography variant="body1" align="left" color="primary">
          Min IVs
        </Typography>
      </Grid>
      <Grid
        container
        item
        spacing={1}
        sm={12}
        md={12}
        xs={12}
        sx={{ justifyContent: { xs: 'center', md: 'flex-start' } }}
      >
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="hp"
            label="HP"
            variant="outlined"
            value={state.minIVs.hp}
            onChange={event =>
              setState(state => ({
                ...state,
                minIVs: {
                  ...state.minIVs,
                  hp: event.target.value,
                },
              }))
            }
          />
        </Grid>
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="atk"
            label="Attack"
            variant="outlined"
            value={state.minIVs.atk}
            onChange={event =>
              setState(state => ({
                ...state,
                minIVs: {
                  ...state.minIVs,
                  atk: event.target.value,
                },
              }))
            }
          />
        </Grid>
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="def"
            label="Defense"
            variant="outlined"
            value={state.minIVs.def}
            onChange={event =>
              setState(state => ({
                ...state,
                minIVs: {
                  ...state.minIVs,
                  def: event.target.value,
                },
              }))
            }
          />
        </Grid>
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="spa"
            label="Special Attack"
            variant="outlined"
            value={state.minIVs.spa}
            onChange={event =>
              setState(state => ({
                ...state,
                minIVs: {
                  ...state.minIVs,
                  spa: event.target.value,
                },
              }))
            }
          />
        </Grid>
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="spd"
            label="Special Defense"
            variant="outlined"
            value={state.minIVs.spd}
            onChange={event =>
              setState(state => ({
                ...state,
                minIVs: {
                  ...state.minIVs,
                  spd: event.target.value,
                },
              }))
            }
          />
        </Grid>
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="spe"
            label="Speed"
            variant="outlined"
            value={state.minIVs.spe}
            onChange={event =>
              setState(state => ({
                ...state,
                minIVs: {
                  ...state.minIVs,
                  spe: event.target.value,
                },
              }))
            }
          />
        </Grid>
      </Grid>
      <Grid item sm={12} md={12} xs={12}>
        <Typography variant="body1" align="left" color="primary">
          Max IVs
        </Typography>
      </Grid>
      <Grid
        container
        item
        spacing={1}
        sm={12}
        md={12}
        xs={12}
        sx={{ justifyContent: { xs: 'center', md: 'flex-start' } }}
      >
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="hp"
            label="HP"
            variant="outlined"
            value={state.maxIVs.hp}
            onChange={event =>
              setState(state => ({
                ...state,
                maxIVs: {
                  ...state.maxIVs,
                  hp: event.target.value,
                },
              }))
            }
          />
        </Grid>
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="atk"
            label="Attack"
            variant="outlined"
            value={state.maxIVs.atk}
            onChange={event =>
              setState(state => ({
                ...state,
                maxIVs: {
                  ...state.maxIVs,
                  atk: event.target.value,
                },
              }))
            }
          />
        </Grid>
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="def"
            label="Defense"
            variant="outlined"
            value={state.maxIVs.def}
            onChange={event =>
              setState(state => ({
                ...state,
                maxIVs: {
                  ...state.maxIVs,
                  def: event.target.value,
                },
              }))
            }
          />
        </Grid>
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="spa"
            label="Special Attack"
            variant="outlined"
            value={state.maxIVs.spa}
            onChange={event =>
              setState(state => ({
                ...state,
                maxIVs: {
                  ...state.maxIVs,
                  spa: event.target.value,
                },
              }))
            }
          />
        </Grid>
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="spd"
            label="Special Defense"
            variant="outlined"
            value={state.maxIVs.spd}
            onChange={event =>
              setState(state => ({
                ...state,
                maxIVs: {
                  ...state.maxIVs,
                  spd: event.target.value,
                },
              }))
            }
          />
        </Grid>
        <Grid item sm={6} md={2} xs={2}>
          <TextField
            inputProps={{
              inputMode: 'numeric',
              pattern: '[0-9]*',
            }}
            fullWidth
            id="spe"
            label="Speed"
            variant="outlined"
            value={state.maxIVs.spe}
            onChange={event =>
              setState(state => ({
                ...state,
                maxIVs: {
                  ...state.maxIVs,
                  spe: event.target.value,
                },
              }))
            }
          />
        </Grid>
      </Grid>
    </React.Fragment>
  );
};
