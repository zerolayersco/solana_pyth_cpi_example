use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

declare_id!("your_program_pubkey_in_base58");

#[program]
pub mod oracle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn get_price(ctx: Context<GetPrice>, maximum_age: u64, feed_id_hex: String) -> Result<()> {
        let feed_id_clean = if feed_id_hex.starts_with("0x") {
            feed_id_hex.trim_start_matches("0x").to_string()
        } else {
            feed_id_hex.clone()
        };
        
        if feed_id_clean.len() != 64 {
            return Err(OracleError::InvalidFeedIdFormat.into());
        }
        
        let feed_id = get_feed_id_from_hex(&feed_id_clean)
            .map_err(|_| OracleError::InvalidFeedIdFormat)?;
        
        let clock = Clock::get().map_err(|_| OracleError::ClockUnavailable)?;
        
        let price = match ctx.accounts.price_update.get_price_no_older_than(
            &clock,
            maximum_age,
            &feed_id
        ) {
            Ok(price) => price,
            Err(_) => {
                return Err(OracleError::PriceUnavailable.into());
            }
        };
        
        msg!("The price is ({} ± {}) * 10^{}", price.price, price.conf, price.exponent);
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// The system program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetPrice<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    // No need for custom constraint - Anchor's typing already ensures it's the right account type
    pub price_update: Account<'info, PriceUpdateV2>,
}

// Custom error type for validation failures
#[error_code]
pub enum OracleError {
    #[msg("Price data is not available for the requested feed")]
    PriceUnavailable,
    
    #[msg("Invalid feed ID format")]
    InvalidFeedIdFormat,
    
    #[msg("Unable to access Solana clock")]
    ClockUnavailable,
}
