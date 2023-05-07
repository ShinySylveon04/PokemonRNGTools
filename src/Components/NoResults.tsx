import React from 'react';
import { TextTableRow } from './TextTableRow';
import { useTranslation } from 'react-i18next';

type Props = {
  colSpan?: number;
  selected?: boolean;
};

export const NoResults = ({ colSpan = 3 }: Props) => {
  const { t } = useTranslation();
  return <TextTableRow colSpan={colSpan}>{t(`No Results Found`)}</TextTableRow>;
};
