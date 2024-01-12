import {
  Context,
  TransactionBuilderGroup,
  some,
  transactionBuilderGroup,
} from '@metaplex-foundation/umi';
import { Extension, getExtensionSerializerFromType } from './extensions';
import { AllocateInstructionAccounts, allocate } from './generated';
import { DEFAULT_CHUNK_SIZE, write } from './write';

export function initialize(
  context: Pick<
    Context,
    'eddsa' | 'identity' | 'payer' | 'programs' | 'transactions'
  >,
  input: AllocateInstructionAccounts & { extension: Extension }
): TransactionBuilderGroup {
  const data = getExtensionSerializerFromType(input.extension.type).serialize(
    input.extension
  );

  const chunked = data.length > DEFAULT_CHUNK_SIZE;

  const builder = allocate(context, {
    ...input,
    extensionType: input.extension.type,
    length: data.length,
    data: chunked ? null : some(data),
  });

  if (chunked) {
    return write(context, {
      ...input,
      data,
    })
      .prepend(builder)
      .sequential();
  }

  return transactionBuilderGroup([builder]);
}
