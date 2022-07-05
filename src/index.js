import React from 'react';
import ReactDOM from 'react-dom';
import { HashRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from './Layouts/Main';
import { Home } from './Pages/Home';
import { Translate } from './Pages/Translate';
import { SwSh } from './SwSh/SwSh';
import { Wild } from './BdSp/Wild/Wild';
import { Stationary } from './BdSp/Stationary/Stationary';
import { Underground } from './BdSp/Underground/Underground';
import { TID } from './BdSp/TID/TID';
import { Roamers } from './BdSp/Roamers/Roamers';

import { MetaTags } from './Components/MetaTags';

import './i18n';

const app = document.getElementById('app');
ReactDOM.render(
  <React.Fragment>
    <MetaTags />
    <HashRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<Home />} />
          <Route path="translate" element={<Translate />} />
          <Route path="swsh" element={<SwSh />} />
          <Route path="bdsp" element={<Wild />} />
          <Route path="bdsp/static" element={<Stationary />} />
          <Route path="bdsp/underground" element={<Underground />} />
          <Route path="bdsp/tid" element={<TID />} />
        </Route>
      </Routes>
    </HashRouter>
  </React.Fragment>,
  app,
);
