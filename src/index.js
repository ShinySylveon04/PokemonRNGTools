import React from 'react';
import ReactDOM from 'react-dom';
import { HashRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from './Layouts/Main';
import { Home } from './Pages/Home';
import { SwSh } from './SwSh/SwSh';
import { Wild } from './BdSp/Wild/Wild';
import { Stationary } from './BdSp/Stationary/Stationary';
import { Underground } from './BdSp/Underground/Underground';

import { MetaTags } from './Components/MetaTags';

const app = document.getElementById('app');
ReactDOM.render(
  <React.Fragment>
    <MetaTags />
    <HashRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<Home />} />
          <Route path="swsh" element={<SwSh />} />
          <Route path="bdsp" element={<Wild />} />
          <Route path="bdsp/stationary" element={<Stationary />} />
          <Route path="bdsp/underground" element={<Underground />} />
        </Route>
      </Routes>
    </HashRouter>
  </React.Fragment>,
  app,
);
