import React, { useState, useEffect } from "react";
import FileUploadButton from "./components/FileUploadButton";
import { Page, Text, useToasts } from "@geist-ui/core";
import { Button, Grid } from "@geist-ui/react";
import { encode_file } from "./scripts/encode";
import { decode_file } from "./scripts/decode";
import FlowComponent from "./diagram";
import { do_check_encode } from "./scripts/check_encode";
import { do_check_decode } from "./scripts/check_decode";

import init from "compression/compression";

const App: React.FC = () => {
  const [file, setFile] = useState<File | null>(null);
  // const [tree, setTree] = useState<string | null>(null);

  useEffect(() => {
    init();
  }, []);

  const { setToast } = useToasts();

  const handleError = (string: String) => {
    setToast({
      text: string,
      type: "error",
    });
  };

  const handleFileSelect = (file: File) => {
    setFile(file);
  };

  const handleEncode = async () => {
    if (!file) {
      return;
    }
    encode_file({ file, handleError });
  };

  const handleDecode = async () => {
    if (!file) {
      return;
    }
    decode_file({ file, handleError });
  };

  return (
    <Page width="1200px">
      <Text h1>Huffman Encoding</Text>

      <Grid.Container gap={2} justify="center">
        <Grid xs={6}>
          <FileUploadButton onFileSelect={handleFileSelect} />
        </Grid>

        <Grid xs={6}>
          {/* @ts-ignore */}
          <Button onClick={handleEncode}>Encode</Button>
        </Grid>

        <Grid xs={6}>
          {/* @ts-ignore */}
          <Button onClick={handleDecode}>Decode</Button>
        </Grid>
        <Grid />
      </Grid.Container>

      <Grid.Container gap={2} justify="center">
        <Text>{file?.name}</Text>
      </Grid.Container>
      {/* <FlowComponent></FlowComponent> */}
    </Page>
  );
};

export default App;
