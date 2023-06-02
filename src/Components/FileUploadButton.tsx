import React from 'react';
import Button, { ButtonProps } from '@mui/material/Button';
import { styled } from '@mui/material/styles';

const HiddenInput = styled('input')({
  display: 'none',
});

type Props = {
  id: string;
  label: string;
  onUpload: (file: Uint8Array) => void;
} & Omit<ButtonProps, 'component' | 'children'>;

export const FileUploadButton = ({
  id,
  label,
  onUpload,
  ...buttonProps
}: Props) => {
  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = event.target.files && event.target.files[0];

    if (selectedFile) {
      const reader = new FileReader();

      reader.onload = e => {
        const content = e.target?.result;
        if (content != null && typeof content !== 'string') {
          const uintArray = new Uint8Array(content);
          onUpload(uintArray);
        }
      };

      reader.readAsArrayBuffer(selectedFile);
    }
  };

  return (
    <>
      <HiddenInput
        accept=".wasm"
        id={id}
        type="file"
        onChange={handleFileChange}
      />
      <label htmlFor={id}>
        <Button {...buttonProps} component="span">
          {label}
        </Button>
      </label>
    </>
  );
};
