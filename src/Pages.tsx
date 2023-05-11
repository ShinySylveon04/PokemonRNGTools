import React from 'react';
import { HashRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from './Layouts/Main';
import { Home } from './Pages/Home';
import { Translate } from './Pages/Translate';
import { ConfiguableSearcher } from './Layouts/ConfiguableSearcher';
import {
  TRANSPORTER_CONFIG,
  BDSP_TID_CONFIG,
  BDSP_UNDERGROUND_CONFIG,
  GEN3_WILD_CONFIG,
  BDSP_WILD_CONFIG,
  BDSP_STATIC_CONFIG,
  SWSH_OVERWORLD_CONFIG,
} from './SearcherConfigs/configs';

export const Pages = () => {
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<Home />} />
          <Route path="translate" element={<Translate />} />
          <Route
            path="gen6/transporter"
            Component={() => (
              <ConfiguableSearcher config={TRANSPORTER_CONFIG} />
            )}
          />
          <Route
            path="bdsp/tid"
            Component={() => <ConfiguableSearcher config={BDSP_TID_CONFIG} />}
          />
          <Route
            path="bdsp/underground"
            Component={() => (
              <ConfiguableSearcher config={BDSP_UNDERGROUND_CONFIG} />
            )}
          />
          <Route
            path="gen3/wild"
            Component={() => <ConfiguableSearcher config={GEN3_WILD_CONFIG} />}
          />
          <Route
            path="bdsp"
            Component={() => <ConfiguableSearcher config={BDSP_WILD_CONFIG} />}
          />
          <Route
            path="bdsp/static"
            Component={() => (
              <ConfiguableSearcher config={BDSP_STATIC_CONFIG} />
            )}
          />
          <Route
            path="swsh"
            Component={() => (
              <ConfiguableSearcher config={SWSH_OVERWORLD_CONFIG} />
            )}
          />
        </Route>
      </Routes>
    </HashRouter>
  );
};
