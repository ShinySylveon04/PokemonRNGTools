import React from 'react';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Button from '@mui/material/Button';
import Alert from '@mui/material/Alert';
import AlertTitle from '@mui/material/AlertTitle';
import { FileUploadButton } from '../Components/FileUploadButton';
import { PluginSearcher } from '../Layouts/PluginSearcher';
import { useTranslation } from 'react-i18next';
import * as wasm from '../../wasm/plugin_template/pkg/chatot_plugin_template';

type WasmState = {
  initializedWasm: typeof wasm | null;
  error: Error | null;
};

export const PluginLoader = () => {
  const { t } = useTranslation();
  const [{ initializedWasm, error }, setWasmState] = React.useState<WasmState>({
    initializedWasm: null,
    error: null,
  });

  return (
    <Box>
      <Toolbar />
      <Typography variant="h3" component="h1" sx={{ mb: 2 }}>
        {t('Chatot Plugins')}
      </Typography>

      <Alert severity="info" sx={{ mb: 2 }}>
        <AlertTitle>{t('Plugins are experimental!')}</AlertTitle>
        {t('You experience may vary while this feature is being built.')}
      </Alert>

      {error != null && (
        <Alert severity="error" sx={{ mb: 2 }}>
          <AlertTitle>
            {t('There was an error loading this plugin.')}
          </AlertTitle>
          {t('Please contact the plugin about this error:')} {error.message}
        </Alert>
      )}

      {initializedWasm == null && (
        <FileUploadButton
          id="wasm-upload"
          variant="contained"
          label={t('Upload plugin')}
          onUpload={data => {
            wasm
              .default(data)
              .then(() => setWasmState({ initializedWasm: wasm, error: null }))
              .catch(error => setWasmState({ initializedWasm: null, error }));
          }}
        />
      )}

      {initializedWasm != null && (
        <>
          <Button
            variant="contained"
            onClick={() => setWasmState({ initializedWasm: null, error: null })}
          >
            {t('Unload plugin')}
          </Button>
          <PluginSearcher
            getFieldGroups={initializedWasm.get_field_groups}
            getResultColumns={initializedWasm.get_result_columns}
            generateResults={initializedWasm.generate_results}
          />
        </>
      )}
    </Box>
  );
};
