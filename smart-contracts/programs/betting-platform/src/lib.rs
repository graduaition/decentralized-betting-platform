use anchor_lang::prelude::*;
declare_id!("ProgramId");

#[program]
pub mod betting_platform {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, neslear: Pubkey, nend: Pubkey, lessly: Pubkey) -> Result<()> {
        let platform = &mut ctx.accounts.platform;
        platform.neslear = neslear;
        platform.nend = nend;
        platform.lessly = lessly;
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64, choice: String) -> Result<()> {
        match choice.as_str() {
            "neslear" => {
                // Transférer funds to the neslear address
            }
            "nend" => {
                // Transférer funds to the nend address
            }
            "lessly" => {
                // Transférer funds to the lessly address
            }
            _ => return Err(ErrorCode::InvalidChoice.into()),
        }
        Ok(())
    }

    pub fn redistribute(ctx: Context<Redistribute>, winner: String) -> Result<()> {
        // Calculate gains and redistribute funds
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 96)]
    pub platform: Account<'info, Platform>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub platform: Account<'info, Platform>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Redistribute<'info> {
    #[account(mut)]
    pub platform: Account<'info, Platform>,
}

#[account]
pub struct Platform {
    pub neslear: Pubkey,
    pub nend: Pubkey,
    pub lessly: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid choice")]
    InvalidChoice,
}
