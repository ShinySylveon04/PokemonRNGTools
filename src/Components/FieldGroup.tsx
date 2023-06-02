import React from 'react';
import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid';
import Paper from '@mui/material/Paper';
import Skeleton from '@mui/material/Skeleton';
import Stack from '@mui/material/Stack';
import { useTranslation } from 'react-i18next';
import { FieldConfig, InputField } from '../Components/InputField';

type LabelComponent = {
  type: 'label';
  label: string;
  id: string;
};

export type FieldComponent = FieldConfig & {
  size?: 'large' | 'small';
};

export type FieldGroupComponent = FieldComponent | LabelComponent;

export type FieldGroup = {
  label: string;
  components: FieldGroupComponent[];
};

type Props =
  | {
      loading?: undefined;
      fieldGroup: FieldGroup;
    }
  | {
      loading: true;
      fieldGroup?: undefined;
    };

export function InputFieldGroup({ fieldGroup, loading }: Props) {
  const { t } = useTranslation();

  return (
    <Paper variant="outlined" sx={{ padding: '10px', m: '10px' }}>
      <Grid
        container
        spacing={2}
        direction="row"
        justifyContent="center"
        alignItems="center"
      >
        <Grid item xs={12}>
          <Typography variant="h6" align="left" color="primary">
            {loading && <Skeleton variant="text" />}
            {!loading && t(fieldGroup.label)}
          </Typography>
        </Grid>

        {loading && (
          <Grid item xs={12}>
            <Skeleton variant="rectangular" height={120} />
          </Grid>
        )}

        {!loading &&
          fieldGroup.components.map(field => {
            if (field.type === 'label') {
              return (
                <Grid key={field.id} item xs={12}>
                  <Typography variant="body1" align="left" color="primary">
                    {t(field.label)}
                  </Typography>
                </Grid>
              );
            }

            return (
              <Grid
                key={field.id}
                item
                sm={field.size === 'small' ? 6 : 3}
                md={field.size === 'small' ? 2 : 3}
                xs={field.size === 'small' ? 4 : 12}
                justifyContent="center"
              >
                <InputField {...field} />
              </Grid>
            );
          })}
      </Grid>
    </Paper>
  );
}
