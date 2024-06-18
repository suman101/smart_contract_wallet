use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};


declare_id!("46SuxeLQ7VFBB1LDK8CK8JJycTkn6LDLPxpF85ii9XxJ");

#[program]
pub mod mass_payouts {
    use super::*;

    pub fn process_mass_payouts(ctx: Context<ProcessMassPayouts>, payouts: Vec<MassPayoutRequest>) -> Result<()> {
        let total_payout_amount = payouts.iter().map(|payout| payout.amount).sum();

        if ctx.accounts.admin_token_account.amount < total_payout_amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }
        
        for payout in payouts {
            anchor_spl::token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        from: ctx.accounts.admin_token_account.to_account_info(),
                        to: ctx.accounts.vendor_token_account.to_account_info(),
                        authority: ctx.accounts.admin.to_account_info(),
                    },
                ),
                payout.amount,
            )?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProcessMassPayouts<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub admin_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vendor_token_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct MassPayoutRequest {
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in the admin's token account")]
    InsufficientFunds,
}



