import type { AccountRewardResponse, CheckDelegationResponse, WithdrawalsResponse } from "@/types/accountTypes";
import type { DelegationResponse } from "@/types/delegationTypes";

import { handleFetch } from "@/lib/handleFetch";
import type { AddressDetailParams } from "./stake";

interface AccountRewardsArgs {
  view: string;
  limit?: number;
  offset?: number;
}

/**
 * Fetches the reward history for a given account from the Cexplorer API.
 *
 * This function retrieves the current configuration using `getCexplorerConfig`,
 * ensures that the `network` is defined, and then sends a request to retrieve
 * the staking rewards for the specified account.
 *
 * @example
 * ```ts
 * const rewards = await getAccountRewards({ view: "stake1u...", limit: 10 });
 * ```
 *
 * @param {AccountRewardsArgs} args - Parameters for the request.
 * @param {number} [args.limit=20] - Maximum number of results to return.
 * @param {number} [args.offset=0] - Number of items to skip (for pagination).
 * @param {string} args.view - The account stake address to fetch rewards for.
 *
 * @returns {Promise<AccountRewardResponse>} A promise resolving to the reward data.
 * @throws If the SDK is not initialized or network is not defined.
 */
export const getAccountRewards = async ({ limit = 20, offset = 0, view }: AccountRewardsArgs) => {
  const url = `/account/reward`;

  const options = {
    params: { limit, offset, view },
  };

  return handleFetch<AccountRewardResponse>(url, offset, options);
};

/**
 * Checks if a user has a delegation for the specified view.
 *
 * Sends a request to the `/account/has_delegation` endpoint to determine
 * whether the user associated with the provided `view` has delegation rights.
 *
 * @param view - The identifier of the view to check delegation for. Can be a string, `undefined`, or `null`.
 * @returns A promise that resolves to a `CheckDelegationResponse` indicating the delegation status.
 */
export const checkUserDelegation = async (view: string | undefined | null) => {
  const url = "/account/has_delegation";

  const options = {
    params: { view },
  };

  return handleFetch<CheckDelegationResponse>(url, undefined, options);
};

/**
 * Fetches the withdrawal history for a given account from the Cexplorer API.
 *
 * This function sends a request to retrieve the withdrawal transactions
 * associated with a specified stake address. Supports pagination through
 * `limit` and `offset` parameters.
 *
 * @example
 * ```ts
 * const data = await getWithdrawals({ view: "stake1u...", limit: 10 });
 * ```
 *
 * @param {AddressDetailParams} args - Parameters for the request.
 * @param {string} args.view - The account stake address to fetch withdrawals for.
 * @param {number} [args.limit=20] - Maximum number of results to return.
 * @param {number} [args.offset=0] - Number of items to skip (for pagination).
 *
 * @returns {Promise<WithdrawalsResponse>} A promise resolving to the withdrawals data.
 * @throws If the request fails or the response is invalid.
 */
export const getWithrawals = async ({ view, limit = 20, offset = 0 }: AddressDetailParams) => {
  const url = `/account/withdrawal`;

  const options = {
    params: {
      view,
      limit,
      offset,
    },
  };

  return handleFetch<WithdrawalsResponse>(url, offset, options);
};

/**
 * Fetches DRep delegation information.
 *
 * This function retrieves the list of DRep delegations, supporting pagination
 * through `limit` and `offset` parameters.
 *
 * @example
 * ```ts
 * const data = await getDelegationVote({ limit: 20, offset: 0 });
 * ```
 *
 * @param {Object} args - Parameters for the request.
 * @param {number} [args.limit=20] - Maximum number of results to return.
 * @param {number} [args.offset=0] - Number of items to skip (for pagination).
 *
 * @returns {Promise<DelegationResponse>} A promise resolving to the DRep delegation data.
 * @throws If the request fails or the response is invalid.
 */
export const getDelegationVote = async ({ limit = 20, offset = 0 }: { limit?: number; offset?: number } = {}) => {
  const url = `/account/delegation_vote`;

  const options = {
    params: {
      limit,
      offset,
    },
  };

  return handleFetch<DelegationResponse>(url, offset, options);
};
