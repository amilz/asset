/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Account,
  Context,
  Pda,
  PublicKey,
  RpcAccount,
  RpcGetAccountOptions,
  RpcGetAccountsOptions,
  assertAccountExists,
  deserializeAccount,
  gpaBuilder,
  publicKey as toPublicKey,
} from '@metaplex-foundation/umi';
import {
  bool,
  publicKey as publicKeySerializer,
  string,
} from '@metaplex-foundation/umi/serializers';
import { AssetAccountData, getAssetAccountDataSerializer } from '../../hooked';
import {
  DelegateArgs,
  Discriminator,
  DiscriminatorArgs,
  StandardArgs,
  StateArgs,
  getDelegateSerializer,
  getDiscriminatorSerializer,
  getStandardSerializer,
  getStateSerializer,
} from '../types';

export type Asset = Account<AssetAccountData>;

export function deserializeAsset(rawAccount: RpcAccount): Asset {
  return deserializeAccount(rawAccount, getAssetAccountDataSerializer());
}

export async function fetchAsset(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<Asset> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  assertAccountExists(maybeAccount, 'Asset');
  return deserializeAsset(maybeAccount);
}

export async function safeFetchAsset(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<Asset | null> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  return maybeAccount.exists ? deserializeAsset(maybeAccount) : null;
}

export async function fetchAllAsset(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<Asset[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, 'Asset');
    return deserializeAsset(maybeAccount);
  });
}

export async function safeFetchAllAsset(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<Asset[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeAsset(maybeAccount as RpcAccount));
}

export function getAssetGpaBuilder(context: Pick<Context, 'rpc' | 'programs'>) {
  const programId = context.programs.getPublicKey(
    'asset',
    'AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      discriminator: DiscriminatorArgs;
      state: StateArgs;
      standard: StandardArgs;
      mutable: boolean;
      holder: PublicKey;
      group: PublicKey;
      authority: PublicKey;
      delegate: DelegateArgs;
      name: string;
    }>({
      discriminator: [0, getDiscriminatorSerializer()],
      state: [1, getStateSerializer()],
      standard: [2, getStandardSerializer()],
      mutable: [3, bool()],
      holder: [4, publicKeySerializer()],
      group: [36, publicKeySerializer()],
      authority: [68, publicKeySerializer()],
      delegate: [100, getDelegateSerializer()],
      name: [133, string({ size: 35 })],
    })
    .deserializeUsing<Asset>((account) => deserializeAsset(account))
    .whereField('discriminator', Discriminator.Asset);
}
