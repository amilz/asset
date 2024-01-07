//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::Standard;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Create {
    /// Asset account
    pub asset: solana_program::pubkey::Pubkey,
    /// The authority of the asset
    pub authority: solana_program::pubkey::Pubkey,
    /// The holder of the asset
    pub holder: solana_program::pubkey::Pubkey,
    /// The account paying for the storage fees
    pub payer: Option<solana_program::pubkey::Pubkey>,
    /// The system program
    pub system_program: Option<solana_program::pubkey::Pubkey>,
}

impl Create {
    pub fn instruction(
        &self,
        args: CreateInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.asset, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.holder,
            false,
        ));
        if let Some(payer) = self.payer {
            accounts.push(solana_program::instruction::AccountMeta::new(payer, true));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        if let Some(system_program) = self.system_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                system_program,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::ASSET_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CreateInstructionData {
    discriminator: u8,
}

impl CreateInstructionData {
    fn new() -> Self {
        Self { discriminator: 1 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateInstructionArgs {
    pub name: String,
    pub standard: Standard,
    pub mutable: bool,
}

/// Instruction builder for `Create`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` asset
///   1. `[]` authority
///   2. `[]` holder
///   3. `[writable, signer, optional]` payer
///   4. `[optional]` system_program
#[derive(Default)]
pub struct CreateBuilder {
    asset: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    holder: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    name: Option<String>,
    standard: Option<Standard>,
    mutable: Option<bool>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Asset account
    #[inline(always)]
    pub fn asset(&mut self, asset: solana_program::pubkey::Pubkey) -> &mut Self {
        self.asset = Some(asset);
        self
    }
    /// The authority of the asset
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// The holder of the asset
    #[inline(always)]
    pub fn holder(&mut self, holder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.holder = Some(holder);
        self
    }
    /// `[optional account]`
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.payer = payer;
        self
    }
    /// `[optional account]`
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.system_program = system_program;
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    /// `[optional argument, defaults to 'Standard::NonFungible']`
    #[inline(always)]
    pub fn standard(&mut self, standard: Standard) -> &mut Self {
        self.standard = Some(standard);
        self
    }
    /// `[optional argument, defaults to 'true']`
    #[inline(always)]
    pub fn mutable(&mut self, mutable: bool) -> &mut Self {
        self.mutable = Some(mutable);
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
        let accounts = Create {
            asset: self.asset.expect("asset is not set"),
            authority: self.authority.expect("authority is not set"),
            holder: self.holder.expect("holder is not set"),
            payer: self.payer,
            system_program: self.system_program,
        };
        let args = CreateInstructionArgs {
            name: self.name.clone().expect("name is not set"),
            standard: self.standard.clone().unwrap_or(Standard::NonFungible),
            mutable: self.mutable.clone().unwrap_or(true),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create` CPI accounts.
pub struct CreateCpiAccounts<'a, 'b> {
    /// Asset account
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the asset
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The holder of the asset
    pub holder: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `create` CPI instruction.
pub struct CreateCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Asset account
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the asset
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The holder of the asset
    pub holder: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: CreateInstructionArgs,
}

impl<'a, 'b> CreateCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateCpiAccounts<'a, 'b>,
        args: CreateInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            asset: accounts.asset,
            authority: accounts.authority,
            holder: accounts.holder,
            payer: accounts.payer,
            system_program: accounts.system_program,
            __args: args,
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
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.asset.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.holder.key,
            false,
        ));
        if let Some(payer) = self.payer {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *payer.key, true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        if let Some(system_program) = self.system_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *system_program.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ASSET_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.asset.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.holder.clone());
        if let Some(payer) = self.payer {
            account_infos.push(payer.clone());
        }
        if let Some(system_program) = self.system_program {
            account_infos.push(system_program.clone());
        }
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

/// Instruction builder for `Create` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` asset
///   1. `[]` authority
///   2. `[]` holder
///   3. `[writable, signer, optional]` payer
///   4. `[optional]` system_program
pub struct CreateCpiBuilder<'a, 'b> {
    instruction: Box<CreateCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateCpiBuilderInstruction {
            __program: program,
            asset: None,
            authority: None,
            holder: None,
            payer: None,
            system_program: None,
            name: None,
            standard: None,
            mutable: None,
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
    /// The authority of the asset
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// The holder of the asset
    #[inline(always)]
    pub fn holder(
        &mut self,
        holder: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.holder = Some(holder);
        self
    }
    /// `[optional account]`
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(
        &mut self,
        payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.payer = payer;
        self
    }
    /// `[optional account]`
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.system_program = system_program;
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.instruction.name = Some(name);
        self
    }
    /// `[optional argument, defaults to 'Standard::NonFungible']`
    #[inline(always)]
    pub fn standard(&mut self, standard: Standard) -> &mut Self {
        self.instruction.standard = Some(standard);
        self
    }
    /// `[optional argument, defaults to 'true']`
    #[inline(always)]
    pub fn mutable(&mut self, mutable: bool) -> &mut Self {
        self.instruction.mutable = Some(mutable);
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
        let args = CreateInstructionArgs {
            name: self.instruction.name.clone().expect("name is not set"),
            standard: self
                .instruction
                .standard
                .clone()
                .unwrap_or(Standard::NonFungible),
            mutable: self.instruction.mutable.clone().unwrap_or(true),
        };
        let instruction = CreateCpi {
            __program: self.instruction.__program,

            asset: self.instruction.asset.expect("asset is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            holder: self.instruction.holder.expect("holder is not set"),

            payer: self.instruction.payer,

            system_program: self.instruction.system_program,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct CreateCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    holder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    name: Option<String>,
    standard: Option<Standard>,
    mutable: Option<bool>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
