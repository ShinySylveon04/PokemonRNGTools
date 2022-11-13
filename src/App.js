import React from 'react';

import { ColorMode } from './Components/ColorMode';
import { Pages } from './Pages';
import { MetaTags } from './Components/MetaTags';

export const App = () => {
  return (
    <React.Fragment>
      <MetaTags />
      <ColorMode>
        <Pages />
      </ColorMode>
    </React.Fragment>
  );
};
