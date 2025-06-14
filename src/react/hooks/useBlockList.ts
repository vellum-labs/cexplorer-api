import { test } from "../../core/client.js";
import { useCexplorerInitGuard } from "../CexplorerProvider.js";

export function useBlocksList(params?: { page?: number; limit?: number }) {
  useCexplorerInitGuard();
  return test("/block");
}
