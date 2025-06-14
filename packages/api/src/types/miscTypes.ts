import type { Rate } from "./blockTypes";
import type { ResponseCore } from "./commonTypes";

export type MiscBasicResponse = {
  code: number;
  data: {
    block: {
      hash: string;
      time: string;
      epoch_no: number;
      block_no: number;
      slot_no: number;
      proto: number;
    };
    ads: {
      data: {
        content: string;
        icon: string;
        section: string;
        title: string;
        type: string;
        link: string;
        text: string;
      };
      type: string;
    }[];
    version: {
      const: number;
      rate: number;
    };
    instance: {
      readonly: boolean;
      server: string;
      snapshot: string;
      time: string;
    };
    rate: Rate;
    rate_day: Rate;
  };
  tokens: number;
  ex: number;
  debug: boolean;
};

export interface BasicRate {
  date: string;
  adausd: number | null;
  btcusd: number | null;
}

export interface MiscRateResponse {
  code: number;
  data: { rates: BasicRate[] };
  tokens: number;
}

interface MiscConstResponseDataEpoch {
  no: number;
  start_time: string;
  end_time: string;
  tx_count: number;
  blk_count: number;
  fees: number;
  out_sum: number;
}

interface MiscConstResponseDataLabelData {
  scriptHash: string;
  contractAddress: string;
}

export interface MiscConstResponseDataLabel {
  name: string;
  type: string;
  label: string;
  data: MiscConstResponseDataLabelData | null;
}

interface MiscConstResponseDataEpochParam {
  nonce: string;
  epoch_no: number;
  influence: string;
  max_epoch: number;
  min_fee_a: number;
  min_fee_b: number;
  price_mem: string;
  price_step: string;
  key_deposit: number;
  max_bh_size: number;
  max_tx_size: number;
  max_val_size: number;
  pool_deposit: number;
  extra_entropy: null;
  max_tx_em_mem: number;
  min_pool_cost: number;
  max_block_size: number;
  min_utxo_size: number;
  protocol_major: number;
  protocol_minor: number;
  max_tx_ex_steps: number;
  decentralisation: number;
  max_block_ex_mem: number;
  collateral_percent: number;
  max_block_ex_steps: number;
  optimal_pool_count: number;
  coint_per_utxo_size: number;
  monetary_expand_rate: string;
  treasury_growth_rate: string;
  max_collateral_inputs: number;
}

interface MiscConstResponseDataEpochStatRewards {
  member: null;
  leader: null;
}

interface MiscConstResponseDataEpochStatStakePools {
  minting: number;
  registered: number;
}

interface MiscConstResponseDataEpochStatStake {
  epoch: number;
  active: number;
  pools: MiscConstResponseDataEpochStatStakePools;
  accounts: number;
}

interface MiscConstResponseDataEpochStatPoots {
  fees: number;
  utxo: number;
  rewards: number;
  slot_no: number;
  block_id: number;
  deposits: number;
  reserves: number;
  treasury: number;
}

interface MiscConstResponseDataEpochStatEpoch {
  start_time: string;
  end_time: string;
  block_count: number;
  tx_count: number;
  fees: number;
  out_sum: number;
  block_size: number;
}

interface MiscConstResponseDataEpochStatPoolStat {
  pools: number;
  pct_leader: null | number;
  pct_member: null | number;
  epoch_stake: null | number;
  delegator_count: null | number;
  delegator_avg: null | number;
}

interface MiscConstResponseDataEpochStatProto {
  min: number;
  max: number;
}
interface MiscConstResponseDataEpochStat {
  epoch_no: number;
  spendable_epoch: number;
  rewards: MiscConstResponseDataEpochStatRewards;
  stake: MiscConstResponseDataEpochStatStake;
  pots: MiscConstResponseDataEpochStatPoots;
  epoch: MiscConstResponseDataEpochStatEpoch;
  pool_stat: MiscConstResponseDataEpochStatPoolStat;
  proto: MiscConstResponseDataEpochStatProto;
  daily: {
    stat: {
      block_version: {
        version: number;
        count: number;
      }[];
    };
  }[];
}

export interface MiscConstResponseData {
  no: number;
  epoch: MiscConstResponseDataEpoch;
  circulating_supply: number;
  labels: MiscConstResponseDataLabel[];
  epoch_param: MiscConstResponseDataEpochParam;
  epoch_stat: MiscConstResponseDataEpochStat;
  live_stake: number | null;
}

interface MiscMarketStatus {
  low: number;
  high: number;
  open: number;
  close: number;
  volume: number;
  time_open: string;
  market_cap: number;
  time_close: string;
}

interface MiscMarketFiat {
  aud: [number, number];
  bgn: [number, number];
  brl: [number, number];
  cad: [number, number];
  chf: [number, number];
  cny: [number, number];
  czk: [number, number];
  dkk: [number, number];
  eur: [number, number];
  gbp: [number, number];
  hkd: [number, number];
  huf: [number, number];
  idr: [number, number];
  ils: [number, number];
  inr: [number, number];
  isk: [number, number];
  jpy: [number, number];
  krw: [number, number];
  mxn: [number, number];
  myr: [number, number];
  nok: [number, number];
  nzd: [number, number];
  php: [number, number];
  pln: [number, number];
  ron: [number, number];
  sek: [number, number];
  sgd: [number, number];
  thb: [number, number];
  try: [number, number];
  usd: [number, number];
  xdr: [number, number];
  zar: [number, number];
}

export interface MiscMarketData {
  date: string;
  epoch_no: number;
  need_fix: string;
  ada: MiscMarketStatus;
  btc: MiscMarketStatus;
  fiat: MiscMarketFiat;
}

interface MiscNew {
  message: {
    [key: string]: string;
  };
}

export interface MiscSearch {
  url: string;
  ident: string;
  title: string;
  category: string;
  extra: {
    icon: "balance" | "time" | "hot" | "promo";
    type: "balance" | "time" | "stake";
    value: number | string;
    id?: string;
  };
}

interface Tier {
  rq_min: number;
  rq_day: number;
  tok_day: number;
  license: string;
}

export interface MiscApiData {
  starter: Tier;
  basic: Tier;
  pro: Tier;
}

export interface Poll {
  name: string;
  url: string;
  applied: string | null;
  date_start: string;
  date_end: string;
  description: string;
  options: string[];
  state: "available" | "closed";
  vote: {
    power: number;
    is_valid: boolean;
    option: string;
    date: string;
  } | null;
  result: Record<string, { count: number; power: number }> | null;
}

export type GroupType = "pool" | "ident" | "asset" | "drep" | "collection";

interface MiscValidate {
  type: string;
  ident: string;
  valid: boolean;
}

export type MiscValidateResponse = ResponseCore<MiscValidate>;
export type PollListResponse = ResponseCore<Poll[]>;
export type MiscConstResponse = ResponseCore<MiscConstResponseData>;
export type MiscMarketResponse = ResponseCore<MiscMarketData>;
export type MiscNewResponse = ResponseCore<MiscNew>;
export type MiscSearchResponse = ResponseCore<MiscSearch[] | MiscSearch>;
export type MiscApiResponse = ResponseCore<{
  plans: MiscApiData;
}>;
