import { test } from "../client.js";

export const getBlock = () => {
  return test("/block");
};
