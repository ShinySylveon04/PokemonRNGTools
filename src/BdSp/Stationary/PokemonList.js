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
    <ListSubheader>Gifts</ListSubheader>
    <MenuItem value={SPECIES.Turtwig}>{t('Turtwig')}</MenuItem>
    <MenuItem value={SPECIES.Chimchar}>{t('Chimchar')}</MenuItem>
    <MenuItem value={SPECIES.Piplup}>{t('Piplup')}</MenuItem>
    <MenuItem value={SPECIES.Eevee}>{t('Eevee')}</MenuItem>
    <MenuItem value={SPECIES.Happiny}>{t('Happiny')}</MenuItem>
    <MenuItem value={SPECIES.Riolu}>{t('Riolu')}</MenuItem>
    <MenuItem value={SPECIES.Mew}>{t('Mew')}</MenuItem>
    <MenuItem value={SPECIES.Jirachi}>{t('Jirachi')}</MenuItem>
    <ListSubheader>Fossils</ListSubheader>
    <MenuItem value={SPECIES.Omanyte}>{t('Omanyte')}</MenuItem>
    <MenuItem value={SPECIES.Kabuto}>{t('Kabuto')}</MenuItem>
    <MenuItem value={SPECIES.Aerodactyl}>{t('Aerodactyl')}</MenuItem>
    <MenuItem value={SPECIES.Lileep}>{t('Lileep')}</MenuItem>
    <MenuItem value={SPECIES.Anorith}>{t('Anorith')}</MenuItem>
    <MenuItem value={SPECIES.Cranidos}>{t('Cranidos')}</MenuItem>
    <MenuItem value={SPECIES.Shieldon}>{t('Shieldon')}</MenuItem>
    <ListSubheader>Stationary</ListSubheader>
    <MenuItem value={SPECIES.Drifloon}>{t('Drifloon')}</MenuItem>
    <MenuItem value={SPECIES.Spiritomb}>{t('Spiritomb')}</MenuItem>
    <MenuItem value={SPECIES.Rotom}>{t('Rotom')}</MenuItem>
    <ListSubheader>Roamers</ListSubheader>
    <MenuItem value={SPECIES.Mesprit}>{t('Mesprit')}</MenuItem>
    <MenuItem value={SPECIES.Cresselia}>{t('Cresselia')}</MenuItem>
    <ListSubheader>Legends</ListSubheader>
    <MenuItem value={SPECIES.Uxie}>{t('Uxie')}</MenuItem>
    <MenuItem value={SPECIES.Azelf}>{t('Azelf')}</MenuItem>
    <MenuItem value={SPECIES.Dialga}>{t('Dialga')}</MenuItem>
    <MenuItem value={SPECIES.Palkia}>{t('Palkia')}</MenuItem>
    <MenuItem value={SPECIES.Heatran}>{t('Heatran')}</MenuItem>
    <MenuItem value={SPECIES.Regigigas}>{t('Regigigas')}</MenuItem>
    <MenuItem value={SPECIES.Giratina}>{t('Giratina')}</MenuItem>
    <ListSubheader>Ramanas Park (Pure Space)</ListSubheader>
    <MenuItem value={SPECIES.Articuno}>{t('Articuno')}</MenuItem>
    <MenuItem value={SPECIES.Zapdos}>{t('Zapdos')}</MenuItem>
    <MenuItem value={SPECIES.Moltres}>{t('Moltres')}</MenuItem>
    <MenuItem value={SPECIES.Raikou}>{t('Raikou')}</MenuItem>
    <MenuItem value={SPECIES.Entei}>{t('Entei')}</MenuItem>
    <MenuItem value={SPECIES.Suicune}>{t('Suicune')}</MenuItem>
    <MenuItem value={SPECIES.Regirock}>{t('Regirock')}</MenuItem>
    <MenuItem value={SPECIES.Regice}>{t('Regice')}</MenuItem>
    <MenuItem value={SPECIES.Registeel}>{t('Registeel')}</MenuItem>
    <MenuItem value={SPECIES.Latias}>{t('Latias')}</MenuItem>
    <MenuItem value={SPECIES.Latios}>{t('Latios')}</MenuItem>
    <ListSubheader>Ramanas Park (Strange Space)</ListSubheader>
    <MenuItem value={SPECIES.Mewtwo}>{t('Mewtwo')}</MenuItem>
    <MenuItem value={SPECIES.Lugia}>{t('Lugia')}</MenuItem>
    <MenuItem value={SPECIES.HoOh}>{t('Ho-Oh')}</MenuItem>
    <MenuItem value={SPECIES.Kyogre}>{t('Kyogre')}</MenuItem>
    <MenuItem value={SPECIES.Groudon}>{t('Groudon')}</MenuItem>
    <MenuItem value={SPECIES.Rayquaza}>{t('Rayquaza')}</MenuItem>
    <ListSubheader>Mythics</ListSubheader>
    <MenuItem value={SPECIES.Darkrai}>{t('Darkrai')}</MenuItem>
    <MenuItem value={SPECIES.Shaymin}>{t('Shaymin')}</MenuItem>
    <MenuItem value={SPECIES.Arceus}>{t('Arceus')}</MenuItem>
  </Select>
);
