import React, { useRef, ChangeEvent } from "react";
import { Button } from "@geist-ui/react";

interface FileUploadButtonProps {
  onFileSelect: (file: File) => void;
}

const FileUploadButton: React.FC<FileUploadButtonProps> = ({
  onFileSelect,
}) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleButtonClick = () => {
    if (fileInputRef.current) {
      fileInputRef.current.click();
    }
  };

  const handleFileChange = (event: ChangeEvent<HTMLInputElement>) => {
    if (event.target.files && event.target.files[0]) {
      onFileSelect(event.target.files[0]);
    }
  };

  return (
    <div>
      <input
        type="file"
        ref={fileInputRef}
        style={{ display: "none" }}
        onChange={handleFileChange}
      />
      {/* @ts-ignore */}
      <Button onClick={handleButtonClick}>Select File</Button>
    </div>
  );
};

export default FileUploadButton;
