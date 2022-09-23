use anchor_lang::prelude::*;

declare_id!("B2tj6s4Nco5rSyEFFqw6Dkc5NsY4upRB5Nntwo5KabiA");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(_ctx: Context<StartStuffOff>) -> Result <()> {
      Ok(())
    }
  }
  
#[derive(Accounts)]
pub struct StartStuffOff {}