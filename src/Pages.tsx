import React from 'react';
import { HashRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from './Layouts/Main';
import { Home } from './Pages/Home';
import { Translate } from './Pages/Translate';
import { PluginLoader } from './Pages/PluginLoader';
import { ChatotSearcher } from './Layouts/ChatotSearcher';
import {
  TRANSPORTER_CONFIG,
  BDSP_TID_CONFIG,
  BDSP_UNDERGROUND_CONFIG,
  GEN3_WILD_CONFIG,
  BDSP_WILD_CONFIG,
  BDSP_STATIC_CONFIG,
  SWSH_OVERWORLD_CONFIG,
} from './searcherConfigs';

export const Pages = () => {
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<Home />} />
          <Route path="translate" element={<Translate />} />
          <Route
            path="gen6/transporter"
            Component={() => <ChatotSearcher config={TRANSPORTER_CONFIG} />}
          />
          <Route
            path="bdsp/tid"
            Component={() => <ChatotSearcher config={BDSP_TID_CONFIG} />}
          />
          <Route
            path="bdsp/underground"
            Component={() => (
              <ChatotSearcher config={BDSP_UNDERGROUND_CONFIG} />
            )}
          />
          <Route
            path="gen3/wild"
            Component={() => <ChatotSearcher config={GEN3_WILD_CONFIG} />}
          />
          <Route
            path="bdsp"
            Component={() => <ChatotSearcher config={BDSP_WILD_CONFIG} />}
          />
          <Route
            path="bdsp/static"
            Component={() => <ChatotSearcher config={BDSP_STATIC_CONFIG} />}
          />
          <Route
            path="swsh"
            Component={() => <ChatotSearcher config={SWSH_OVERWORLD_CONFIG} />}
          />
          <Route path="plugin" Component={() => <PluginLoader />} />
        </Route>
      </Routes>
    </HashRouter>
  );
};
