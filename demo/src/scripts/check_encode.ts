import { check_encode } from "compression/compression";
import { ProcessFileInterface } from "./ProcessFileInterface";

export const do_check_encode = async ({
  file,
  handleError,
}: ProcessFileInterface): Promise<boolean> => {
  const arrayBuffer = await file.arrayBuffer();
  const uint8Array = new Uint8Array(arrayBuffer);

  try {
    check_encode(uint8Array);
  } catch (e) {
    if (typeof e === "string") {
      handleError(e);
    } else if (e instanceof Error) {
      handleError(e.message);
    }
    return false;
  }
  return true;
};
