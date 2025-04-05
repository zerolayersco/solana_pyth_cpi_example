use anchor_lang::prelude::*;
use oracle::program::Oracle;
use oracle::cpi::accounts::GetPrice;

declare_id!("your_program_pubkey_in_base58");

#[program]
pub mod price_fetcher {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn fetch_price(ctx: Context<FetchPrice>, maximum_age: u64, feed_id_hex: String) -> Result<()> {
        let feed_id_clean = if feed_id_hex.starts_with("0x") {
            feed_id_hex.trim_start_matches("0x")
        } else {
            &feed_id_hex
        };
        
        if feed_id_clean.len() != 64 || !is_valid_hex_string(feed_id_clean) {
            return Err(PriceFetcherError::InvalidFeedIdFormat.into());
        }
        
        if maximum_age == 0 || maximum_age > 3600 {  // Example: limit to 1 hour max
            return Err(PriceFetcherError::InvalidMaximumAge.into());
        }
        
        // Capture the initial state of the price_update account for post-CPI validation
        let initial_owner = ctx.accounts.price_update.owner;
        let initial_data_len = ctx.accounts.price_update.data_len();
        let initial_data_is_empty = ctx.accounts.price_update.data_is_empty();
        
        // Pre-CPI validation
        if initial_owner == ctx.program_id {
            return Err(PriceFetcherError::InvalidPriceAccountOwner.into());
        }
        
        msg!("Price Fetcher is invoking Oracle program to get price data...");
        
        oracle::cpi::get_price(
            CpiContext::new(
                ctx.accounts.oracle_program.to_account_info(),
                GetPrice {
                    payer: ctx.accounts.payer.to_account_info(),
                    price_update: ctx.accounts.price_update.to_account_info(),
                },
            ),
            maximum_age,
            feed_id_hex,
        ).map_err(|err| {
            msg!("Oracle program returned an error: {:?}", err);
            PriceFetcherError::OracleProgramError
        })?;
        
        // Post-CPI validation to prevent reentrancy attacks
        // Verify that critical account properties didn't change
        if ctx.accounts.price_update.owner != initial_owner {
            return Err(PriceFetcherError::AccountStateModified.into());
        }
        
        // Check data length hasn't been maliciously modified
        if ctx.accounts.price_update.data_len() != initial_data_len {
            return Err(PriceFetcherError::AccountStateModified.into());
        }
        
        // Check emptiness state hasn't changed
        if ctx.accounts.price_update.data_is_empty() != initial_data_is_empty {
            return Err(PriceFetcherError::AccountStateModified.into());
        }
        
        msg!("Price successfully fetched from Oracle program!");
        
        Ok(())
    }
}

// Helper function to validate hex string
fn is_valid_hex_string(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// The system program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FetchPrice<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: This account is used by the Oracle program to read price data.
    /// We perform basic validation but rely on the Oracle program for detailed checks.
    #[account(
        // Verify account is not empty
        constraint = !price_update.data_is_empty() @ PriceFetcherError::EmptyPriceAccount
    )]
    pub price_update: AccountInfo<'info>,
    
    /// The Oracle program that we will invoke
    #[account(address = oracle::ID)]
    pub oracle_program: Program<'info, Oracle>,
}

// Custom error type for price_fetcher validation
#[error_code]
pub enum PriceFetcherError {
    #[msg("Price update account contains no data")]
    EmptyPriceAccount,
    
    #[msg("Invalid feed ID format (must be 64 hex characters)")]
    InvalidFeedIdFormat,
    
    #[msg("Invalid maximum age parameter")]
    InvalidMaximumAge,
    
    #[msg("Oracle program returned an error")]
    OracleProgramError,
    
    #[msg("Invalid price account owner")]
    InvalidPriceAccountOwner,
    
    #[msg("Account state was unexpectedly modified during external call")]
    AccountStateModified,
}
