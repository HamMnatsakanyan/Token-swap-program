pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("G2oivvA9V2pnTXiNmCkyLiGwjjTXprUP7cnmZb83KMrk");

#[program]
pub mod swap_program {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>, 
        id: u64, 
        token_a_offered_amount: u64, 
        token_b_wanted_amount: u64
    ) -> Result<()> {

        instructions::make_offer::send_offered_tokens_to_vault(&ctx, token_a_offered_amount)?;
        instructions::make_offer::save_offer(ctx, id, token_b_wanted_amount)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()>{

        instructions::take_offer::send_wanted_tokens_to_maker(&ctx)?;
        instructions::take_offer::withdraw_and_close_vault(ctx)
    }
}
