export interface PoolMeta {
  ticker: string;
  name: string;
  description: string;
  extended: null;
  homepage: string;
}

export interface PoolInfo {
  id: string;
  meta: PoolMeta | null;
}
