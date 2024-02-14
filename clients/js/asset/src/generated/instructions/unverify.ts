/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type UnverifyInstructionAccounts = {
  /** Asset account */
  asset: PublicKey | Pda;
  /** Creator account to unverify */
  creator: Signer;
};

// Data.
export type UnverifyInstructionData = { discriminator: number };

export type UnverifyInstructionDataArgs = {};

export function getUnverifyInstructionDataSerializer(): Serializer<
  UnverifyInstructionDataArgs,
  UnverifyInstructionData
> {
  return mapSerializer<
    UnverifyInstructionDataArgs,
    any,
    UnverifyInstructionData
  >(
    struct<UnverifyInstructionData>([['discriminator', u8()]], {
      description: 'UnverifyInstructionData',
    }),
    (value) => ({ ...value, discriminator: 9 })
  ) as Serializer<UnverifyInstructionDataArgs, UnverifyInstructionData>;
}

// Instruction.
export function unverify(
  context: Pick<Context, 'programs'>,
  input: UnverifyInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'asset',
    'AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'
  );

  // Accounts.
  const resolvedAccounts = {
    asset: {
      index: 0,
      isWritable: true as boolean,
      value: input.asset ?? null,
    },
    creator: {
      index: 1,
      isWritable: false as boolean,
      value: input.creator ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getUnverifyInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
