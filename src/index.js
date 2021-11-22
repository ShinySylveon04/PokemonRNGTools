import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { SwSh } from './SwSh/SwSh';
import { BdSp } from './BdSp/BdSp';

const app = document.getElementById('app');
ReactDOM.render(
  <BrowserRouter basename={process.env.PUBLIC_URL}>
    <Routes>
      <Route path="/" element={<SwSh />} />
      <Route path="bdsp" element={<BdSp />} />
    </Routes>
  </BrowserRouter>,
  app,
);
