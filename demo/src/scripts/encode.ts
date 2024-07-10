import { huff_encode } from "compression/compression";
import { ProcessFileInterface } from "./ProcessFileInterface";

export const encode_file = async ({
  file,
  handleError,
}: ProcessFileInterface) => {
  const arrayBuffer = await file.arrayBuffer();
  const uint8Array = new Uint8Array(arrayBuffer);
  let result: Uint8Array | null = null;

  try {
    result = huff_encode(uint8Array);
  } catch (e) {
    if (typeof e === "string") {
      handleError(e); // works, `e` narrowed to string
    } else if (e instanceof Error) {
      handleError(e.message); // works, `e` narrowed to Error
    }
  }

  if (result instanceof Uint8Array) {
    const blob = new Blob([result.buffer], {
      type: "application/octet-stream",
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = file?.name.replace(/\.[^/.]+$/, "") + ".huff"; // Change the file name and extension as needed
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }
};
