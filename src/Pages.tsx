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
import { TRANSPORTER_CONFIG } from './SearcherConfigs/Transporter';
import { BDSP_TID_CONFIG } from './SearcherConfigs/BdSpTid';
import { BDSP_UNDERGROUND_CONFIG } from './SearcherConfigs/BdSpUnderground';
import { GEN3_WILD_CONFIG } from './SearcherConfigs/Gen3Wild';
import { BDSP_WILD_CONFIG } from './SearcherConfigs/BdSpWild';
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
            element={<ConfiguableSearcher config={TRANSPORTER_CONFIG} />}
          />
          <Route
            path="bdsp/tid2"
            element={<ConfiguableSearcher config={BDSP_TID_CONFIG} />}
          />
          <Route
            path="bdsp/underground2"
            element={<ConfiguableSearcher config={BDSP_UNDERGROUND_CONFIG} />}
          />
          <Route
            path="gen3/wild2"
            element={<ConfiguableSearcher config={GEN3_WILD_CONFIG} />}
          />
          <Route
            path="bdsp2"
            element={<ConfiguableSearcher config={BDSP_WILD_CONFIG} />}
          />
        </Route>
      </Routes>
    </HashRouter>
  );
};
