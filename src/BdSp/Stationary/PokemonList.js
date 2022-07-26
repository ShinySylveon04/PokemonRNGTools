import React from 'react';
import ListSubheader from '@mui/material/ListSubheader';
import MenuItem from '@mui/material/MenuItem';
import Select from '@mui/material/Select';

import { SPECIES } from '../../Utils/species';
import { RNG_CONFIG } from '../../Utils/rng_config';

export const PokemonList = ({ t, setState, state }) => (
  <Select
    labelId="pokemon-type-label"
    id="pokemon"
    value={state.pokemon}
    label={t('Pokemon')}
    onChange={event => {
      setState(state => ({
        ...state,
        ...RNG_CONFIG[event.target.value],
        pokemon: event.target.value,
      }));
    }}
  >
    <ListSubheader>{t('Gifts')}</ListSubheader>
    <MenuItem value={SPECIES.Turtwig}>
      {t(`pokemonNames.${SPECIES.Turtwig}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Chimchar}>
      {t(`pokemonNames.${SPECIES.Chimchar}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Piplup}>
      {t(`pokemonNames.${SPECIES.Piplup}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Eevee}>
      {t(`pokemonNames.${SPECIES.Eevee}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Happiny}>
      {t(`pokemonNames.${SPECIES.Happiny}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Riolu}>
      {t(`pokemonNames.${SPECIES.Riolu}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Mew}>{t(`pokemonNames.${SPECIES.Mew}`)}</MenuItem>
    <MenuItem value={SPECIES.Jirachi}>
      {t(`pokemonNames.${SPECIES.Jirachi}`)}
    </MenuItem>
    <ListSubheader>{t('Fossils')}</ListSubheader>
    <MenuItem value={SPECIES.Omanyte}>
      {t(`pokemonNames.${SPECIES.Omanyte}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Kabuto}>
      {t(`pokemonNames.${SPECIES.Kabuto}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Aerodactyl}>
      {t(`pokemonNames.${SPECIES.Aerodactyl}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Lileep}>
      {t(`pokemonNames.${SPECIES.Lileep}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Anorith}>
      {t(`pokemonNames.${SPECIES.Anorith}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Cranidos}>
      {t(`pokemonNames.${SPECIES.Cranidos}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Shieldon}>
      {t(`pokemonNames.${SPECIES.Shieldon}`)}
    </MenuItem>
    <ListSubheader>{t('Stationary')}</ListSubheader>
    <MenuItem value={SPECIES.Drifloon}>
      {t(`pokemonNames.${SPECIES.Drifloon}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Spiritomb}>
      {t(`pokemonNames.${SPECIES.Spiritomb}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Rotom}>
      {t(`pokemonNames.${SPECIES.Rotom}`)}
    </MenuItem>
    <ListSubheader>{t('Roamers')}</ListSubheader>
    <MenuItem value={SPECIES.Mesprit}>
      {t(`pokemonNames.${SPECIES.Mesprit}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Cresselia}>
      {t(`pokemonNames.${SPECIES.Cresselia}`)}
    </MenuItem>
    <ListSubheader>{t('Legends')}</ListSubheader>
    <MenuItem value={SPECIES.Uxie}>
      {t(`pokemonNames.${SPECIES.Uxie}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Azelf}>
      {t(`pokemonNames.${SPECIES.Azelf}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Dialga}>
      {t(`pokemonNames.${SPECIES.Dialga}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Palkia}>
      {t(`pokemonNames.${SPECIES.Palkia}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Heatran}>
      {t(`pokemonNames.${SPECIES.Heatran}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Regigigas}>
      {t(`pokemonNames.${SPECIES.Regigigas}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Giratina}>
      {t(`pokemonNames.${SPECIES.Giratina}`)}
    </MenuItem>
    <ListSubheader>{t('Ramanas Park (Pure Space)')}</ListSubheader>
    <MenuItem value={SPECIES.Articuno}>
      {t(`pokemonNames.${SPECIES.Articuno}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Zapdos}>
      {t(`pokemonNames.${SPECIES.Zapdos}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Moltres}>
      {t(`pokemonNames.${SPECIES.Moltres}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Raikou}>
      {t(`pokemonNames.${SPECIES.Raikou}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Entei}>
      {t(`pokemonNames.${SPECIES.Entei}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Suicune}>
      {t(`pokemonNames.${SPECIES.Suicune}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Regirock}>
      {t(`pokemonNames.${SPECIES.Regirock}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Regice}>
      {t(`pokemonNames.${SPECIES.Regice}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Registeel}>
      {t(`pokemonNames.${SPECIES.Registeel}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Latias}>
      {t(`pokemonNames.${SPECIES.Latias}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Latios}>
      {t(`pokemonNames.${SPECIES.Latios}`)}
    </MenuItem>
    <ListSubheader>{t('Ramanas Park (Strange Space)')}</ListSubheader>
    <MenuItem value={SPECIES.Mewtwo}>
      {t(`pokemonNames.${SPECIES.Mewtwo}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Lugia}>
      {t(`pokemonNames.${SPECIES.Lugia}`)}
    </MenuItem>
    <MenuItem value={SPECIES.HoOh}>
      {t(`pokemonNames.${SPECIES.HoOh}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Kyogre}>
      {t(`pokemonNames.${SPECIES.Kyogre}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Groudon}>
      {t(`pokemonNames.${SPECIES.Groudon}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Rayquaza}>
      {t(`pokemonNames.${SPECIES.Rayquaza}`)}
    </MenuItem>
    <ListSubheader>{t('Mythics')}</ListSubheader>
    <MenuItem value={SPECIES.Darkrai}>
      {t(`pokemonNames.${SPECIES.Darkrai}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Shaymin}>
      {t(`pokemonNames.${SPECIES.Shaymin}`)}
    </MenuItem>
    <MenuItem value={SPECIES.Arceus}>
      {t(`pokemonNames.${SPECIES.Arceus}`)}
    </MenuItem>
  </Select>
);
