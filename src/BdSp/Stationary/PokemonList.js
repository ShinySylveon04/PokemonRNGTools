import React from 'react';
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

export const PokemonList = ({ setState, state }) => {
  const { t } = useTranslation();

  const SpeciesMenuItem = ({ speciesList }) => {
    return speciesList.map(speciesId => (
      <option key={speciesId} value={speciesId}>
        {t(`pokemonNames.${speciesId}`)}
      </option>
    ));
  };
  return (
    <React.Fragment>
      <FormControl fullWidth>
        <InputLabel htmlFor="pokemon-type-label">{t('Pokemon')}</InputLabel>
        <Select
          native
          inputProps={{
            name: 'pokemon',
            id: 'pokemon-type-label',
          }}
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
          <optgroup label={t('Gifts')}>
            <SpeciesMenuItem speciesList={GIFT_POKEMON} />
          </optgroup>
          <optgroup label={t('Fossils')}>
            <SpeciesMenuItem speciesList={FOSSIL_POKEMON} />
          </optgroup>
          <optgroup label={t('Stationary')}>
            <SpeciesMenuItem speciesList={STATIONARY_POKEMON} />
          </optgroup>
          <optgroup label={t('Roamers')}>
            <SpeciesMenuItem speciesList={ROAMER_POKEMON} />
          </optgroup>
          <optgroup label={t('Legends')}>
            <SpeciesMenuItem speciesList={LEGENDS_POKEMON} />
          </optgroup>
          <optgroup label={t('Ramanas Park (Pure Space)')}>
            <SpeciesMenuItem speciesList={PURE_SPACE_POKEMON} />
          </optgroup>
          <optgroup label={t('Ramanas Park (Strange Space)')}>
            <SpeciesMenuItem speciesList={STRANGE_SPACE_POKEMON} />
          </optgroup>
          <optgroup label={t('Mythics')}>
            <SpeciesMenuItem speciesList={MYTHICS_POKEMON} />
          </optgroup>
        </Select>
      </FormControl>
    </React.Fragment>
  );
};
