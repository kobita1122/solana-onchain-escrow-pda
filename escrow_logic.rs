use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

// 

declare_id!("Escrow11111111111111111111111111111111111111");

#[program]
pub mod solana_escrow {
    use super::*;

    pub fn initialize_escrow(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.maker_token_account.to_account_info(),
            to: ctx.accounts.vault_account.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mut)]
    pub maker_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = 8 + 32 + 32 + 8,
        seeds = [b"escrow", maker.key().as_ref()],
        bump
    )]
    pub vault_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
