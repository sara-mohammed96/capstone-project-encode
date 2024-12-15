#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod capstoneprojectencode {
    use super::*;

  pub fn close(_ctx: Context<CloseCapstoneprojectencode>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.capstoneprojectencode.count = ctx.accounts.capstoneprojectencode.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.capstoneprojectencode.count = ctx.accounts.capstoneprojectencode.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeCapstoneprojectencode>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.capstoneprojectencode.count = value.clone();
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
pub struct Update<'info> {
  #[account(mut)]
  pub capstoneprojectencode: Account<'info, Capstoneprojectencode>,
}

#[account]
#[derive(InitSpace)]
pub struct Capstoneprojectencode {
  count: u8,
}
