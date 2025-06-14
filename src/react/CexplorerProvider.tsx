import React, { createContext, useContext, useEffect, useState } from "react";
import { initCexplorer, CexplorerConfig } from "../core/config.js";

const CexplorerInitContext = createContext<boolean>(false);

export function CexplorerProvider({ children, config }: { children: React.ReactNode; config: CexplorerConfig }) {
  const [ready, setReady] = useState(false);

  useEffect(() => {
    initCexplorer(config);
    setReady(true);
  }, [config.network]);

  return <CexplorerInitContext.Provider value={ready}>{children}</CexplorerInitContext.Provider>;
}

export function useCexplorerInitGuard() {
  const ready = useContext(CexplorerInitContext);
  if (!ready) {
    throw new Error("Cexplorer not initialized. Wrap your app in <CexplorerProvider config={...}>");
  }
}
