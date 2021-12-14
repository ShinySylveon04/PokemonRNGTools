import React from 'react';
import { Helmet } from 'react-helmet';
import CHATOT_PNG from '../Resources/Chatot.png';

export const MetaTags = ({
  title = 'Chatot',
  description = 'Web based RNG tools for Pokemon',
}) => {
  return (
    <Helmet>
      <title>{title}</title>
      <meta name="title" content={title} />
      <meta name="description" content={description} />
      <link rel="icon" type="image/png" href={CHATOT_PNG} />

      {/* Facebook */}
      <meta property="og:type" content="website" />
      <meta property="og:title" content={title} />
      <meta property="og:description" content={description} />
      <meta property="og:image" content={CHATOT_PNG} />

      {/* Twitter */}
      <meta property="twitter:card" content="summary_large_image" />
      <meta property="twitter:title" content={title} />
      <meta property="twitter:description" content={description} />
      <meta property="twitter:image" content={CHATOT_PNG} />
    </Helmet>
  );
};
