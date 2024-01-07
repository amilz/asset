//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Transfer {
    /// Asset account
    pub asset: solana_program::pubkey::Pubkey,
    /// Current holder of the asset or transfer delegate
    pub signer: solana_program::pubkey::Pubkey,
    /// The recipient of the asset
    pub recipient: solana_program::pubkey::Pubkey,
}

impl Transfer {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.asset, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.recipient,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = TransferInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::ASSET_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct TransferInstructionData {
    discriminator: u8,
}

impl TransferInstructionData {
    fn new() -> Self {
        Self { discriminator: 5 }
    }
}

/// Instruction builder for `Transfer`.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` signer
///   2. `[]` recipient
#[derive(Default)]
pub struct TransferBuilder {
    asset: Option<solana_program::pubkey::Pubkey>,
    signer: Option<solana_program::pubkey::Pubkey>,
    recipient: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl TransferBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Asset account
    #[inline(always)]
    pub fn asset(&mut self, asset: solana_program::pubkey::Pubkey) -> &mut Self {
        self.asset = Some(asset);
        self
    }
    /// Current holder of the asset or transfer delegate
    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
        self
    }
    /// The recipient of the asset
    #[inline(always)]
    pub fn recipient(&mut self, recipient: solana_program::pubkey::Pubkey) -> &mut Self {
        self.recipient = Some(recipient);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Transfer {
            asset: self.asset.expect("asset is not set"),
            signer: self.signer.expect("signer is not set"),
            recipient: self.recipient.expect("recipient is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `transfer` CPI accounts.
pub struct TransferCpiAccounts<'a, 'b> {
    /// Asset account
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Current holder of the asset or transfer delegate
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The recipient of the asset
    pub recipient: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `transfer` CPI instruction.
pub struct TransferCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Asset account
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Current holder of the asset or transfer delegate
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The recipient of the asset
    pub recipient: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> TransferCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: TransferCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            asset: accounts.asset,
            signer: accounts.signer,
            recipient: accounts.recipient,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.asset.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.signer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.recipient.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = TransferInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ASSET_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.asset.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.recipient.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Transfer` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` signer
///   2. `[]` recipient
pub struct TransferCpiBuilder<'a, 'b> {
    instruction: Box<TransferCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> TransferCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(TransferCpiBuilderInstruction {
            __program: program,
            asset: None,
            signer: None,
            recipient: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Asset account
    #[inline(always)]
    pub fn asset(&mut self, asset: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.asset = Some(asset);
        self
    }
    /// Current holder of the asset or transfer delegate
    #[inline(always)]
    pub fn signer(
        &mut self,
        signer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.signer = Some(signer);
        self
    }
    /// The recipient of the asset
    #[inline(always)]
    pub fn recipient(
        &mut self,
        recipient: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.recipient = Some(recipient);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = TransferCpi {
            __program: self.instruction.__program,

            asset: self.instruction.asset.expect("asset is not set"),

            signer: self.instruction.signer.expect("signer is not set"),

            recipient: self.instruction.recipient.expect("recipient is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct TransferCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    recipient: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
