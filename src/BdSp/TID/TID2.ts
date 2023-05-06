import { calculateTID } from './TID';
import { SearcherConfig } from '../../Layouts/ConfiguableSearcher';

export const BDSP_TID_CONFIG: SearcherConfig = {
  fieldGroups: [
    {
      label: 'RNG Info',
      components: [
        {
          type: 'hex_number',
          id: 'seed_0',
          label: 'Seed 0',
          defaultValue: '',
        },
        {
          type: 'hex_number',
          id: 'seed_1',
          label: 'Seed 1',
          defaultValue: '',
        },
        {
          type: 'hex_number',
          id: 'seed_2',
          label: 'Seed 2',
          defaultValue: '',
        },
        {
          type: 'hex_number',
          id: 'seed_3',
          label: 'Seed 3',
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
      ],
    },
    {
      label: 'Filters',
      components: [
        {
          type: 'select',
          size: 'small',
          id: 'id_type',
          defaultValue: 'None',
          label: 'ID Filter',
          options: ['None', 'TID', 'SID', 'TSV', 'G8TID'],
        },
        {
          type: 'text',
          size: 'small',
          id: 'ids',
          defaultValue: '',
          required: false,
          label: 'IDs',
        },
      ],
    },
  ],
  resultColumns: ['Advances', 'Gen 8 TID', 'TID', 'SID', 'TSV'],
  generateResults: async values => {
    const parsedSettings = {
      rng_state: [
        values.seed_0,
        values.seed_1,
        values.seed_2,
        values.seed_3,
      ].map(num => parseInt(num, 16)),
      min_advances: parseInt(values.min_advances, 10),
      max_advances: parseInt(values.max_advances, 10),
      id: values.ids
        .split('\n')
        .filter(id => id.trim().length !== 0)
        .map(id => parseInt(id, 10)),
      filter_type: values.id_type,
    };

    const results = await calculateTID(parsedSettings);
    return results.map(result => {
      return [
        result.advances.toString(),
        result.g8tid.toString(),
        result.tid.toString(),
        result.sid.toString(),
        result.tsv.toString(),
      ];
    });
  },
};
