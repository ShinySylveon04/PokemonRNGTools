import React from 'react';
import { HashRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from './Layouts/Main';
import { Home } from './Pages/Home';
import { Translate } from './Pages/Translate';
import { SwSh } from './SwSh/SwSh';
import { Wild } from './BdSp/Wild/Wild';
import { Stationary } from './BdSp/Stationary/Stationary';
import { Underground } from './BdSp/Underground/Underground';
import { TID } from './BdSp/TID/TID';
import { Gen3Wild } from './Gen3/Wild';
import { Gen6Transporter } from './Gen6/Transporter';
import { ConfiguableSearcher } from './Layouts/ConfiguableSearcher';

export const Pages = () => {
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<Home />} />
          <Route path="translate" element={<Translate />} />
          <Route path="gen3/wild" element={<Gen3Wild />} />
          <Route path="gen6/transporter" element={<Gen6Transporter />} />
          <Route path="swsh" element={<SwSh />} />
          <Route path="bdsp" element={<Wild />} />
          <Route path="bdsp/static" element={<Stationary />} />
          <Route path="bdsp/underground" element={<Underground />} />
          <Route path="bdsp/tid" element={<TID />} />
          <Route
            path="gen6/transporter2"
            element={
              <ConfiguableSearcher
                fieldGroups={[
                  {
                    label: 'RNG Info',
                    components: [
                      {
                        type: 'hex_number',
                        onChange: () => {},
                        config: {
                          label: 'Seed',
                        },
                      },
                      {
                        type: 'number',
                        onChange: () => {},
                        defaultValue: 0,
                        config: {
                          label: 'Min Advances',
                        },
                      },
                      {
                        type: 'number',
                        onChange: () => {},
                        defaultValue: 10000,
                        config: {
                          label: 'Max Advances',
                        },
                      },
                      {
                        type: 'number',
                        onChange: () => {},
                        defaultValue: 0,
                        config: {
                          label: 'Delay',
                        },
                      },
                      {
                        type: 'number',
                        onChange: () => {},
                        defaultValue: 0,
                        config: {
                          label: 'TID',
                        },
                      },
                      {
                        type: 'checkbox',
                        onChange: () => {},
                        config: {
                          label: 'Shiny Pokemon',
                        },
                      },
                      {
                        type: 'checkbox',
                        onChange: () => {},
                        config: {
                          label: 'Mew or Celebi',
                        },
                      },
                    ],
                  },
                  {
                    label: 'Filters',
                    components: [
                      {
                        type: 'label',
                        label: 'Min IVs',
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 0,
                        config: {
                          label: 'HP',
                        },
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 0,
                        config: {
                          label: 'Attack',
                        },
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 0,
                        config: {
                          label: 'Defense',
                        },
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 0,
                        config: {
                          label: 'Special Attack',
                        },
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 0,
                        config: {
                          label: 'Special Defense',
                        },
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 0,
                        config: {
                          label: 'Speed',
                        },
                      },
                      {
                        type: 'label',
                        label: 'Max IVs',
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 31,
                        config: {
                          label: 'HP',
                        },
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 31,
                        config: {
                          label: 'Attack',
                        },
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 31,
                        config: {
                          label: 'Defense',
                        },
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 31,
                        config: {
                          label: 'Special Attack',
                        },
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 31,
                        config: {
                          label: 'Special Defense',
                        },
                      },
                      {
                        type: 'number',
                        size: 'small',
                        onChange: () => {},
                        defaultValue: 31,
                        config: {
                          label: 'Speed',
                        },
                      },
                    ],
                  },
                ]}
                resultColumns={[
                  'Advances',
                  'IVs',
                  'Hidden Power',
                  'PSV',
                  'PID',
                ]}
                generateResults={() => {
                  return new Array(100)
                    .fill(0)
                    .map((_, i) => [
                      i.toString(10),
                      '10/31/29/5/30/4',
                      'Fighting',
                      '1234',
                      'AABBCCDD',
                    ]);
                }}
              />
            }
          />
        </Route>
      </Routes>
    </HashRouter>
  );
};
