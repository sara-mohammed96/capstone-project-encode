#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_lang::accounts::account::Account;
use solana_program::pubkey::Pubkey;
use solana_program::vote::state::{VoteState, VoteInit};

declare_id!("5rwPBBSHeHrhBAN7PCDxg7aqA1zrS4PzZS7Mg3b5ainm");
// declare_id!("2hdUghEmtvvHjJpe6npchQQrbiCJZCXuX9B1Yno8FpN9");

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
        // for vote_account_info in vote_accounts.iter() {

            let vote_state = VoteState::deserialize(&mut &vote_accounts.data.borrow_mut()[..]).unwrap();
            // let data = vote_accounts.try_borrow_data()
            // .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotSerialize)?;
    
            // // Deserialize the vote state
            // let vote_state = VoteState::deserialize(&mut &data[..])
            // .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotSerialize)?;
    
            msg!("Validator: {:?}", vote_state.node_pubkey);
        // }

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
    /// CHECK: purposefully unsafe
    #[account(address = solana_program::vote::program::ID)]
    pub vote_accounts: AccountInfo<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Capstoneprojectencode {
    count: u8,
}
