import { calculatePokemon as calculateTransporterPokemon } from './Transporter';
import { SearcherConfig } from '../Layouts/ConfiguableSearcher';
import { formatIVs } from '../Utils/formatIVs';

export const TRANSPORTER_CONFIG: SearcherConfig = {
  fieldGroups: [
    {
      label: 'RNG Info',
      components: [
        {
          type: 'hex_number',
          id: 'seed',
          label: 'Seed',
          defaultValue: '',
        },
        {
          type: 'number',
          id: 'min_advances',
          defaultValue: '0',
          label: 'Min Advances',
        },
        {
          type: 'number',
          id: 'max_advances',
          defaultValue: '10000',
          label: 'Max Advances',
        },
        {
          type: 'number',
          id: 'delay',
          defaultValue: '0',
          label: 'Delay',
        },
        {
          type: 'number',
          id: 'tid',
          defaultValue: '0',
          label: 'TID',
        },
        {
          type: 'checkbox',
          id: 'shiny_pokemon',
          label: 'Shiny Pokemon',
          defaultValue: 'false',
        },
        {
          type: 'checkbox',
          id: 'mew_or_celebi',
          label: 'Mew or Celebi',
          defaultValue: 'false',
        },
      ],
    },
    {
      label: 'Filters',
      components: [
        {
          type: 'label',
          label: 'Min IVs',
          id: 'min_ivs_label',
        },
        {
          type: 'number',
          size: 'small',
          id: 'min_hp_iv',
          defaultValue: '0',
          label: 'HP',
        },
        {
          type: 'number',
          size: 'small',
          id: 'min_atk_iv',
          defaultValue: '0',
          label: 'Attack',
        },
        {
          type: 'number',
          size: 'small',
          id: 'min_def_iv',
          defaultValue: '0',
          label: 'Defense',
        },
        {
          type: 'number',
          size: 'small',
          id: 'min_spa_iv',
          defaultValue: '0',
          label: 'Special Attack',
        },
        {
          type: 'number',
          size: 'small',
          id: 'min_spd_iv',
          defaultValue: '0',
          label: 'Special Defense',
        },
        {
          type: 'number',
          size: 'small',
          id: 'min_spe_iv',
          defaultValue: '0',
          label: 'Speed',
        },
        {
          type: 'label',
          label: 'Max IVs',
          id: 'max_ivs_label',
        },
        {
          type: 'number',
          size: 'small',
          id: 'max_hp_iv',
          defaultValue: '31',
          label: 'HP',
        },
        {
          type: 'number',
          size: 'small',
          id: 'max_atk_iv',
          defaultValue: '31',
          label: 'Attack',
        },
        {
          type: 'number',
          size: 'small',
          id: 'max_def_iv',
          defaultValue: '31',
          label: 'Defense',
        },
        {
          type: 'number',
          size: 'small',
          id: 'max_spa_iv',
          defaultValue: '31',
          label: 'Special Attack',
        },
        {
          type: 'number',
          size: 'small',
          id: 'max_spd_iv',
          defaultValue: '31',
          label: 'Special Defense',
        },
        {
          type: 'number',
          size: 'small',
          id: 'max_spe_iv',
          defaultValue: '31',
          label: 'Speed',
        },
      ],
    },
  ],
  resultColumns: ['Advances', 'IVs', 'Hidden Power', 'PSV', 'PID'],
  generateResults: async values => {
    const parsedSettings = {
      min_ivs: [
        parseInt(values.min_hp_iv, 10),
        parseInt(values.min_atk_iv, 10),
        parseInt(values.min_def_iv, 10),
        parseInt(values.min_spa_iv, 10),
        parseInt(values.min_spd_iv, 10),
        parseInt(values.min_spe_iv, 10),
      ],
      max_ivs: [
        parseInt(values.max_hp_iv, 10),
        parseInt(values.max_atk_iv, 10),
        parseInt(values.max_def_iv, 10),
        parseInt(values.max_spa_iv, 10),
        parseInt(values.max_spd_iv, 10),
        parseInt(values.max_spe_iv, 10),
      ],
      rng_state: parseInt(values.seed, 16),
      min_advances: parseInt(values.min_advances, 10),
      max_advances: parseInt(values.max_advances, 10),
      delay: parseInt(values.delay, 10),
      iv_rolls: values.mew_or_celebi === 'true',
      is_shiny: values.shiny_pokemon === 'true',
      tid: parseInt(values.tid, 10),
    };

    const results = await calculateTransporterPokemon(parsedSettings);
    const formattedResults = results.map(result => {
      return [
        result.advances.toString(),
        formatIVs(result.ivs),
        result.hidden_power,
        result.psv.toString(),
        result.pid.toString(16),
      ];
    });
    return formattedResults;
  },
};
