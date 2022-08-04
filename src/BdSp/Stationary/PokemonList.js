import React from 'react';
import ListSubheader from '@mui/material/ListSubheader';
import MenuItem from '@mui/material/MenuItem';
import Select from '@mui/material/Select';
import InputLabel from '@mui/material/InputLabel';
import FormControl from '@mui/material/FormControl';
import { useTranslation } from 'react-i18next';

import { SPECIES } from '../../Utils/species';
import { RNG_CONFIG } from '../../Utils/rng_config';

const GIFT_POKEMON = [
  SPECIES.Turtwig,
  SPECIES.Chimchar,
  SPECIES.Piplup,
  SPECIES.Eevee,
  SPECIES.Happiny,
  SPECIES.Riolu,
  SPECIES.Mew,
  SPECIES.Jirachi,
];

const FOSSIL_POKEMON = [
  SPECIES.Omanyte,
  SPECIES.Kabuto,
  SPECIES.Aerodactyl,
  SPECIES.Lileep,
  SPECIES.Anorith,
  SPECIES.Cranidos,
  SPECIES.Shieldon,
];

const STATIONARY_POKEMON = [SPECIES.Drifloon, SPECIES.Spiritomb, SPECIES.Rotom];

const ROAMER_POKEMON = [SPECIES.Mesprit, SPECIES.Cresselia];

const LEGENDS_POKEMON = [
  SPECIES.Uxie,
  SPECIES.Azelf,
  SPECIES.Dialga,
  SPECIES.Palkia,
  SPECIES.Heatran,
  SPECIES.Regigigas,
  SPECIES.Giratina,
];

const PURE_SPACE_POKEMON = [
  SPECIES.Articuno,
  SPECIES.Zapdos,
  SPECIES.Moltres,
  SPECIES.Raikou,
  SPECIES.Entei,
  SPECIES.Suicune,
  SPECIES.Regirock,
  SPECIES.Regice,
  SPECIES.Registeel,
  SPECIES.Latias,
  SPECIES.Latios,
];

const STRANGE_SPACE_POKEMON = [
  SPECIES.Mewtwo,
  SPECIES.Lugia,
  SPECIES.HoOh,
  SPECIES.Kyogre,
  SPECIES.Groudon,
  SPECIES.Rayquaza,
];

const MYTHICS_POKEMON = [SPECIES.Darkrai, SPECIES.Shaymin, SPECIES.Arceus];

const SpeciesMenuItem = ({ speciesList }) => {
  const { t } = useTranslation();
  return speciesList.map(speciesId => (
    <MenuItem key={speciesId} value={speciesId}>
      {t(`pokemonNames.${speciesId}`)}
    </MenuItem>
  ));
};

export const PokemonList = ({ t, setState, state }) => {
  return (
    <React.Fragment>
      <FormControl fullWidth>
        <InputLabel id="pokemon-type-label">{t('Pokemon')}</InputLabel>
        <Select
          labelId="pokemon-type-label"
          id="pokemon"
          value={state.pokemon}
          label={t('Pokemon')}
          onChange={event => {
            console.log(event);
            setState(state => ({
              ...state,
              ...RNG_CONFIG[event.target.value],
              pokemon: event.target.value,
            }));
          }}
        >
          <ListSubheader>{t('Gifts')}</ListSubheader>
          <SpeciesMenuItem speciesList={GIFT_POKEMON} />
          <ListSubheader>{t('Fossils')}</ListSubheader>
          <SpeciesMenuItem speciesList={FOSSIL_POKEMON} />
          <ListSubheader>{t('Stationary')}</ListSubheader>
          <SpeciesMenuItem speciesList={STATIONARY_POKEMON} />
          <ListSubheader>{t('Roamers')}</ListSubheader>
          <SpeciesMenuItem speciesList={ROAMER_POKEMON} />
          <ListSubheader>{t('Legends')}</ListSubheader>
          <SpeciesMenuItem speciesList={LEGENDS_POKEMON} />
          <ListSubheader>{t('Ramanas Park (Pure Space)')}</ListSubheader>
          <SpeciesMenuItem speciesList={PURE_SPACE_POKEMON} />
          <ListSubheader>{t('Ramanas Park (Strange Space)')}</ListSubheader>
          <SpeciesMenuItem speciesList={STRANGE_SPACE_POKEMON} />
          <ListSubheader>{t('Mythics')}</ListSubheader>
          <SpeciesMenuItem speciesList={MYTHICS_POKEMON} />
        </Select>
      </FormControl>
    </React.Fragment>
  );
};
