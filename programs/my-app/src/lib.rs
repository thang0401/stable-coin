use anchor_lang::prelude::*;
use anchor_spl::token::{ self, MintTo, Burn, Mint, TokenAccount, Token };
declare_id!("5qnhaAU1wQ1uLEbMQudtLHfhtUTtpbzEvDffZwGoCDhz");
#[program]
pub mod sfcvnd {
    use super::*;
    const OWNER: &str = "ECTFcqDfC74T7kuyLzjpWdMZCiwLWwZHoSw6LCGgNkZk";
    pub fn lock_target(ctx: Context<TargetUser>, targetkey: Pubkey) -> Result<()> {
        if targetkey == Pubkey::default() {
            return Err(ErrorCode::InvalidTargetKey.into());
        };
        ctx.accounts.target.asset_target = targetkey;
        Ok(())
    }
    pub fn init_user(ctx: Context<CreateUser>) -> Result<()> {
        if ctx.accounts.client.asset_account != 0 {
            return Err(ErrorCode::AccountNotEmpty.into());
        } else {
            ctx.accounts.client.asset_account = 0;
        }
        Ok(())
    }
    pub fn clear_user(ctx: Context<DeleteUser>) -> Result<()> {
        if ctx.accounts.client.asset_account != 0 {
            return Err(ErrorCode::AccountNotEmpty.into());
        } else {
            let _ = ctx.accounts.client.close(ctx.accounts.sol_destination.to_account_info());
        }
        Ok(())
    }
    pub fn withdraw_asset(ctx: Context<FixAccount>, amount: u64) -> Result<()> {
        if ctx.accounts.signer.key().to_string() != OWNER {
            return Err(ErrorCode::Unauthorized.into());
        }
        if amount < 10000 {
            return Err(ErrorCode::InvalidAmount.into());
        }
        if ctx.accounts.client.asset_account < amount {
            return Err(ErrorCode::NotEnoughVND.into());
        } else {
            ctx.accounts.client.asset_account -= amount;
            msg!("You withdraw {} VND. You have {} VND left.", amount, ctx.accounts.client.asset_account);
        }
        Ok(())
    }
    pub fn deposit_asset(ctx: Context<FixAccount>, amount: u64) -> Result<()> {
        if ctx.accounts.signer.key().to_string() != OWNER {
            return Err(ErrorCode::Unauthorized.into());
        }
        if amount < 10000 {
            return Err(ErrorCode::InvalidAmount.into());
        }
        ctx.accounts.client.asset_account += amount;
        msg!("You deposit {} VND. You now have {} VND.", amount, ctx.accounts.client.asset_account);
        Ok(())
    }
    pub fn tranfer_asset(ctx: Context<TranferAccount>, amount: u64) -> Result<()> {
        if ctx.accounts.fromclient.asset_account < amount {
            return Err(ErrorCode::NotEnoughVND.into());
        } else {
            ctx.accounts.fromclient.asset_account -= amount;
            ctx.accounts.toclient.asset_account += amount;
            msg!("You tranfer {} VND. ", amount);
            msg!("To Pubkey: {}. ", ctx.accounts.target.asset_target);
        }
        Ok(())
    }
    pub fn tribute_asset(ctx: Context<VaultAccountTribute>, amount: u64, bump: u64) -> Result<()> {
        if ctx.accounts.donator.asset_account < amount {
            return Err(ErrorCode::NotEnoughVND.into());
        } else {
            ctx.accounts.donator.asset_account -= amount;
            ctx.accounts.vault.asset_account += amount;
            msg!("You tribute {} VND. ", amount);
            msg!("To the vault");
            let seeds = &[b"vault".as_ref(), &[bump.try_into().unwrap()]];
            let signer_seeds = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.tk.to_account_info(),
                    authority: ctx.accounts.authority.clone(),
                },
                signer_seeds,
            );
            token::mint_to(cpi_ctx, amount * 100000)?;
        }
        Ok(())
    }
    pub fn summon_asset(ctx: Context<VaultAccountSummon>, amount: u64, is_target: bool) -> Result<()> {
        if ctx.accounts.tk.amount < amount * 100000 {
            return Err(ErrorCode::NotEnoughSFCVND.into());
        } else {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token.to_account_info(),
                Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.tk.to_account_info(),
                    authority: ctx.accounts.signer.to_account_info(),
                },
            );
            token::burn(cpi_ctx, amount * 100000)?;
            ctx.accounts.vault.asset_account -= amount;
            if is_target {
                ctx.accounts.client.asset_account += amount;
            } else {
                ctx.accounts.donator.asset_account += amount;
            }
            msg!("You summon {} VND. ", amount);
            msg!("From the vault");
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct TargetUser<'info> {
    #[account( 
        init_if_needed, 
        payer = signer,
        space = 1000,
        seeds = [b"target", signer.key().as_ref()],
        bump,
    )]
    pub target: Account<'info, UserTarget>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account( 
        init_if_needed, 
        payer = signer,
        space = 1000,
        seeds = [b"client", signer.key().as_ref()],
        bump,
    )]
    pub client: Account<'info, UserInfor>,
    #[account( 
        init_if_needed, 
        payer = signer,
        space = 1000,
        seeds = [b"vault"],
        bump,
    )]
    pub vault: Account<'info, UserInfor>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DeleteUser<'info> {
    #[account(
        mut,
        close = sol_destination,
        seeds = [b"client", signer.key().as_ref()],
        bump,
    )]
    pub client: Account<'info, UserInfor>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub sol_destination: Signer<'info>,
}
#[derive(Accounts)]
pub struct FixAccount<'info> {
    #[account( 
        mut,
        seeds = [b"target", signer.key().as_ref()],
        bump,
    )]
    pub target: Account<'info, UserTarget>,
    #[account(
        mut,
        seeds = [b"client", target.asset_target.to_bytes().as_ref()],
        bump,
    )]
    pub client: Account<'info, UserInfor>,
    #[account(mut)]
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct VaultAccountTribute<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    pub vault: Account<'info, UserInfor>,
    #[account(
        mut,
        seeds = [b"client", signer.key().as_ref()],
        bump,
    )]
    pub donator: Account<'info, UserInfor>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub tk: Account<'info, TokenAccount>,
    pub authority: AccountInfo<'info>,
    pub token: Program<'info, Token>,
}
#[derive(Accounts)]
pub struct VaultAccountSummon<'info> {
    #[account( 
        mut,
        seeds = [b"target", signer.key().as_ref()],
        bump,
    )]
    pub target: Account<'info, UserTarget>,
    #[account(
        mut,
        seeds = [b"client", target.asset_target.to_bytes().as_ref()],
        bump,
    )]
    pub client: Account<'info, UserInfor>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    pub vault: Account<'info, UserInfor>,
    #[account(
        mut,
        seeds = [b"client", signer.key().as_ref()],
        bump,
    )]
    pub donator: Account<'info, UserInfor>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub tk: Account<'info, TokenAccount>,
    pub token: Program<'info, Token>,
}
#[derive(Accounts)]
pub struct TranferAccount<'info> {
    #[account( 
        mut,
        seeds = [b"target", signer.key().as_ref()],
        bump,
    )]
    pub target: Account<'info, UserTarget>,
    #[account(
        mut,
        seeds = [b"client", target.asset_target.to_bytes().as_ref()],
        bump,
    )]
    pub toclient: Account<'info, UserInfor>,
    #[account(
        mut,
        seeds = [b"client", signer.key().as_ref()],
        bump,
    )]
    pub fromclient: Account<'info, UserInfor>,
    #[account(mut)]
    pub signer: Signer<'info>,
}
#[account]
pub struct UserInfor {
    pub asset_account: u64,
}
#[account]
pub struct UserTarget {
    pub asset_target: Pubkey,
}
#[error_code]
pub enum ErrorCode {
    #[msg("Not enough VND")]
    NotEnoughVND,
    #[msg("Not enough SFC - VND")]
    NotEnoughSFCVND,
    #[msg("Account not empty")]
    AccountNotEmpty,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid target key")]
    InvalidTargetKey,
    #[msg("Account already initialized")]
    AccountAlreadyInitialized,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("InvalidAuthority")]
    InvalidAuthority,
}