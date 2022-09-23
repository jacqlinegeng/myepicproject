use anchor_lang::prelude::*;

declare_id!("B2tj6s4Nco5rSyEFFqw6Dkc5NsY4upRB5Nntwo5KabiA");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(_ctx: Context<StartStuffOff>) -> Result <()> {
      let base_account = &mut _ctx.accounts.base_account;
      base_account.total_gifs = 0;
      Ok(())
    }

    pub fn add_gif(_ctx: Context<AddGif>) -> Result <()> {
      let base_account = &mut _ctx.accounts.base_account;
      base_account.total_gifs += 1;
      Ok(())
    }
  }
  
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}