use anchor_lang::prelude::error_code;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use solana_program::system_instruction;
pub const VAULT_SEED: &[u8] = b"vault";
pub const TOKEN_SEED: &[u8] = b"token";
pub const PDA_SEED: &[u8] = b"pda_seed";
declare_id!("47BiiSo5Hv7PhbuQ3gdKBtF9gsCx46YUDPqD26rAu7PR");

#[program]
pub mod presale {
    use super::*;
    pub fn inititialize(ctx: Context<Initialize>, rate: u64, presale_amount: u64) -> Result<()> {
        let presale = &mut ctx.accounts.presale_details;
        require!(!presale.is_active, PresaleError::PresaleInitialized);
        presale.rate = rate;
        presale.authority = ctx.accounts.authority.key();
        presale.is_active = true;
        presale.token_mint = ctx.accounts.mint.key();
        //transfer initial tokens to program(pda)
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_vault_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            presale_amount,
            ctx.accounts.mint.decimals,
        )?;
        Ok(())
    }
    pub fn refund_presale(ctx: Context<Refund>, presale_amount: u64) -> Result<()> {
        let presale = &mut ctx.accounts.presale_details;
        require!(
            presale.authority == *ctx.accounts.authority.key,
            PresaleError::Unauthorized
        );
        //transfer tokens to program(pda)
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_vault_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            presale_amount,
            ctx.accounts.mint.decimals,
        )?;
        Ok(())
    }
    pub fn set_rate(ctx: Context<SetRate>, rate: u64) -> Result<()> {
        let presale = &mut ctx.accounts.presale_details;
        require!(
            presale.authority == *ctx.accounts.authority.key,
            PresaleError::Unauthorized
        );
        presale.rate = rate;
        Ok(())
    }
    pub fn buy_token(ctx: Context<BuyToken>, sol_amount: u64) -> Result<()> {
        let presale = &mut ctx.accounts.presale_details;
        let buyer = &ctx.accounts.signer;
        require!(presale.is_active, PresaleError::PresaleInactive);
        // transfer sol
        let transfer_instruction =
            system_instruction::transfer(&buyer.key(), &presale.key(), sol_amount);
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                ctx.accounts.signer.to_account_info(),
                presale.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
        msg!(" sol_amount: {}", sol_amount);
        msg!(" presale_rate: {}", presale.rate);
        let presale_amount = sol_amount.checked_mul(presale.rate).unwrap();
        msg!(" presale_amount: {}", presale_amount);
        let bump = ctx.bumps.token_vault_account;
        let signer: &[&[&[u8]]] = &[&[VAULT_SEED, &[bump]]];
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.token_vault_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.token_vault_account.to_account_info(),
                },
                signer,
            ),
            presale_amount,
            ctx.accounts.mint.decimals,
        )?;
        presale.total_sol_raised = presale.total_sol_raised.checked_add(sol_amount).unwrap();
        Ok(())
    }
    pub fn withdraw_token(ctx: Context<WithdrawToken>) -> Result<()> {
        let presale = &ctx.accounts.presale_details;
        // require!(!presale.is_active, PresaleError::PresaleInactive);
        require!(
            presale.authority == *ctx.accounts.signer.key,
            PresaleError::Unauthorized
        );
        let token_account = &ctx.accounts.token_vault_account;
        let vault_balance = token_account.amount;
        let bump = ctx.bumps.token_vault_account;
        let signer: &[&[&[u8]]] = &[&[VAULT_SEED, &[bump]]];
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.token_vault_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.token_vault_account.to_account_info(),
                },
                signer,
            ),
            vault_balance,
            ctx.accounts.mint.decimals,
        )?;
        Ok(())
    }
    pub fn withdraw_sol(ctx: Context<WithdrawSol>) -> Result<()> {
        let presale = &mut ctx.accounts.presale_details;
        let owner = &ctx.accounts.authority;
        require!(
            presale.authority == *ctx.accounts.authority.key,
            PresaleError::Unauthorized
        );
        let amount = presale.to_account_info().lamports();
        // let lamports = presale.get_lamports();
        msg!(" presale_amount: {}", amount);
        **presale.to_account_info().try_borrow_mut_lamports()? -= amount - 1;
        **owner.try_borrow_mut_lamports()? += amount - 1;
        presale.total_sol_raised = presale.to_account_info().lamports();
        Ok(())
    }
    pub fn get_pda_token_balance(ctx: Context<GetPdaTokenBalance>) -> Result<()> {
        let token_account = &ctx.accounts.token_vault_account;
        let balance = token_account.amount;
        msg!("PDA Token Balance: {}", balance);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    // #[account(mut, has_one = authority)]
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 +std::mem::size_of::<PresaleDetails>(),
        seeds = [PDA_SEED],
        bump,
    )]
    pub presale_details: Account<'info, PresaleDetails>,
    #[account(
    init_if_needed, 
        seeds = [VAULT_SEED],
        bump,
        payer = authority,
        token::mint = mint,
        token::authority = token_vault_account,
    )]
    pub token_vault_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    // #[account(mut, has_one = authority)]
    #[account(
        mut,
        seeds = [PDA_SEED],
        bump,
    )]
    pub presale_details: Account<'info, PresaleDetails>,
    #[account(
    mut, 
        seeds = [VAULT_SEED],
        bump,
    )]
    pub token_vault_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut, 
        seeds = [VAULT_SEED],
        bump,
    )]
    pub token_vault_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
         init_if_needed,
         payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [PDA_SEED],
        bump,
    )]
    pub presale_details: Account<'info, PresaleDetails>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct WithdrawToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut, 
        seeds = [VAULT_SEED],
        bump,
    )]
    pub token_vault_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [PDA_SEED],
        bump,
    )]
    pub presale_details: Account<'info, PresaleDetails>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [PDA_SEED],
        bump,
        has_one = authority 
    )]
    pub presale_details: Account<'info, PresaleDetails>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SetRate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [PDA_SEED],
        bump,
    )]
    pub presale_details: Account<'info, PresaleDetails>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct GetPdaTokenBalance<'info> {
    #[account(
        mut, 
        seeds = [VAULT_SEED],
        bump,
    )]
    pub token_vault_account: InterfaceAccount<'info, TokenAccount>,
}
#[account]
pub struct PresaleDetails {
    pub rate: u64,
    pub token_mint: Pubkey,
    pub authority: Pubkey,
    pub is_active: bool,
    pub total_sol_raised: u64,
}
#[error_code]
pub enum PresaleError {
    #[msg("Presale Inactive.")]
    PresaleInactive,
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Insufficient funds in presale")]
    InsufficientFunds,
    #[msg("Presale Initialized")]
    PresaleInitialized,
}

