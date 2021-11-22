import React from 'react';
import ReactDOM from 'react-dom';
import { HashRouter, Routes, Route } from 'react-router-dom';
import { SwSh } from './SwSh/SwSh';
import { BdSp } from './BdSp/BdSp';

const app = document.getElementById('app');
ReactDOM.render(
  <HashRouter>
    <Routes>
      <Route path="/" element={<SwSh />} />
      <Route path="bdsp" element={<BdSp />} />
    </Routes>
  </HashRouter>,
  app,
);
