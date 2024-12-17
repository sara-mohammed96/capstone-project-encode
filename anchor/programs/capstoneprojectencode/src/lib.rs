#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("2hdUghEmtvvHjJpe6npchQQrbiCJZCXuX9B1Yno8FpN9");

#[program]
pub mod capstoneprojectencode {
  use super::*;

  pub fn close(_ctx: Context<CloseCapstoneprojectencode>) -> Result<()> {
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeCapstoneprojectencode>) -> Result<()> {
    Ok(())
  }

  pub fn fetch_validators(ctx: Context<FetchValidators>) -> Result<()> {
      let vote_accounts = &ctx.accounts.vote_accounts;

      // Example: Iterate through vote accounts and print validator details
      for vote_account_info in vote_accounts.iter() {
          let vote_state = VoteState::deserialize(&mut &vote_account_info.data[..]).unwrap();
          msg!("Validator: {:?}", vote_state.node_pubkey);
      }

      Ok(())
  }

}

#[derive(Accounts)]
pub struct InitializeCapstoneprojectencode<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Capstoneprojectencode::INIT_SPACE,
  payer = payer
  )]
  pub capstoneprojectencode: Account<'info, Capstoneprojectencode>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseCapstoneprojectencode<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub capstoneprojectencode: Account<'info, Capstoneprojectencode>,
}

#[derive(Accounts)]
pub struct FetchValidators<'info> {
    /// A generic account list to represent vote accounts
    #[account(address = solana_program::vote::program::ID)]
    pub vote_accounts: AccountInfo<'info>,
}

//#[account]
//#[derive(InitSpace)]
//pub struct Capstoneprojectencode {
//  count: u8,
//}
