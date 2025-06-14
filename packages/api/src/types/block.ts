import type { ResponseCore } from "./common";
import type { PoolInfo } from "./pool";

export interface Block {
  block_no: number;
  time: string;
  hash: string;
  epoch_no: number;
  slot_no: number;
  epoch_slot_no: number;
  size: number;
  proto_minor: number;
  proto_major: number;
  op_cert_counter: number;
  vrf_key: string;
  tx_count: number;
  pool: PoolInfo;
  epoch_param: {
    max_block_size: number;
    protocol_major: number;
    protocol_minor: number;
    max_block_ex_mem: number;
    max_block_ex_steps: number;
  };
}

interface BlockList {
  count: number;
  data: Block[];
}

export type BlockListResponse = ResponseCore<BlockList>;
export type BlockListProps = {
  limit?: number;
  offset?: number;
  pool_id?: string;
  epoch_no?: number;
  hash?: string;
  slot_no?: number;
  block_no?: number;
};
