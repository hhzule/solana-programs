use solana_program::clock::Clock;
use solana_program::system_instruction;
use {
    anchor_lang::prelude::*,
    anchor_lang::system_program::{transfer as Systransfer, Transfer as SysTransfer},
    anchor_spl::{
        // associated_token,
        associated_token::AssociatedToken,
        token::spl_token::instruction::AuthorityType,
        token::{set_authority, transfer, Mint, SetAuthority, Token, TokenAccount, Transfer},
    },
};

declare_id!("529jaB5PAPBaoDjtyoD4y55K8B11jWcFY878iQtUtoyt");
pub mod constants {
    pub const TOKEN_SEED: &[u8] = b"vault";
    pub const NFT_INFO_SEED: &[u8] = b"nft_info";
    pub const PDA_SEED: &[u8] = b"marketplace_pda_seed";
}

#[program]
pub mod nft_market {
    use super::*;
    pub fn create_item(ctx: Context<CreateItem>, amount: u64, fund_lamports: u64) -> Result<()> {
        msg!("Transferring NFT...");
        let nft_info = &mut ctx.accounts.nft_info_account;
        let owner_tk_acc = &ctx.accounts.owner_token_account;
        let user = &ctx.accounts.owner_authority;
        let nft_mint = &ctx.accounts.mint;

        if nft_mint.supply != 1 {
            return Err(ErrorCode::NotAnNFT.into()); //check if it is single nft
        }
        if owner_tk_acc.amount != 1 {
            return Err(ErrorCode::NotAvaiable.into());
        }
        if owner_tk_acc.mint != nft_mint.key() {
            return Err(ErrorCode::WrongMintAccount.into()); //check if account belongs to mint
        }
        if owner_tk_acc.owner != user.key() {
            return Err(ErrorCode::MustBeOwnerOfNFT.into()); //check if user owns account
        }
        //crate acc and transfer fund
        let pda = &mut ctx.accounts.pda_account;
        let signer = &mut ctx.accounts.owner_authority;
        let system_program = &ctx.accounts.system_program;
        let pda_balance_before = pda.get_lamports();
        Systransfer(
            CpiContext::new(
                system_program.to_account_info(),
                SysTransfer {
                    from: signer.to_account_info(),
                    to: pda.to_account_info(),
                },
            ),
            fund_lamports,
        )?;

        let pda_balance_after = pda.get_lamports();

        require_eq!(pda_balance_after, pda_balance_before + fund_lamports);

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                current_authority: ctx.accounts.owner_authority.to_account_info(),
                account_or_mint: ctx.accounts.owner_token_account.to_account_info(),
            },
        );
        set_authority(
            cpi_context,
            AuthorityType::AccountOwner, // authority type is AccountOwner
            Some(ctx.accounts.pda_account.key()),
        )?;
        let clock = Clock::get()?;
        nft_info.created_at = clock.slot;
        nft_info.owner = ctx.accounts.owner_authority.key();
        nft_info.price = amount;
        nft_info.sold = false;
        nft_info.is_listed = true;
        nft_info.mint = nft_mint.key();
        msg!("NFT transferred successfully.");

        Ok(())
    }
    pub fn remove_item(ctx: Context<RemoveItem>) -> Result<()> {
        msg!("Transferring NFT...");
        let nft_info = &mut ctx.accounts.nft_info_account;
        let owner_tk_acc = &ctx.accounts.owner_token_account;
        let mint = ctx.accounts.mint.key();
        let user = ctx.accounts.signer.key();
        if nft_info.owner.key() != user.key() {
            return Err(ErrorCode::MustBeOwnerOfNFT.into()); // check the owner of nft
        }
        if nft_info.is_listed != true {
            return Err(ErrorCode::NFTNotListed.into()); // check if nft is listed
        }
        if nft_info.sold != false {
            return Err(ErrorCode::NFTSold.into()); // check if nft is sold
        }
        if owner_tk_acc.mint != mint.key() {
            return Err(ErrorCode::WrongMintAccount.into()); // //check if account belongs to mint
        }

        let bump_seed = ctx.bumps.pda_account;
        let signer_seeds: &[&[&[u8]]] = &[&[constants::PDA_SEED, &[bump_seed]]];
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                current_authority: ctx.accounts.pda_account.to_account_info(),
                account_or_mint: ctx.accounts.owner_token_account.to_account_info(),
            },
            signer_seeds,
        );
        set_authority(
            cpi_context,
            AuthorityType::AccountOwner, // authority type is AccountOwner
            Some(nft_info.owner),
        )?;

        nft_info.is_listed = false;
        msg!("NFT transferred successfully.");
        Ok(())
    }
    pub fn purchase_item(ctx: Context<PurchaseItem>, amount: u64) -> Result<()> {
        let nft_info = &mut ctx.accounts.nft_info_account;
        let buyer = &ctx.accounts.signer;
        if amount < nft_info.price {
            return Err(ErrorCode::WrongAmount.into());
        }
        if nft_info.is_listed != true {
            return Err(ErrorCode::NFTNotListed.into()); // check if nft is listed
        }
        if nft_info.sold == true {
            return Err(ErrorCode::NFTSold.into()); // check if nft is sold
        }
        msg!("Transferring NFT...");

        let bump = &[ctx.bumps.pda_account];
        let seeds: &[&[u8]] = &[constants::PDA_SEED, bump];
        let signer_seeds = &[&seeds[..]];

        // // transfer sol to owner
        let transfer_instruction =
            system_instruction::transfer(&buyer.key(), &nft_info.owner.key(), nft_info.price);
        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.nft_owner.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.owner_token_account.to_account_info(),
                    to: ctx.accounts.buyer_token_account.to_account_info(),
                    authority: ctx.accounts.pda_account.to_account_info(),
                },
                signer_seeds,
            ),
            1,
        )?;
        nft_info.sold = true;
        nft_info.owner = buyer.key();
        nft_info.is_listed = false;
        msg!("NFT transferred successfully.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateItem<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    #[account(mut,
        associated_token::mint = mint,
        associated_token::authority = owner_authority,
      )]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(
    mut,
    seeds = [constants::PDA_SEED],
    bump
    )]
    pub pda_account: SystemAccount<'info>,
    #[account(
     init_if_needed,
     seeds = [constants::NFT_INFO_SEED, mint.key().as_ref()],
     bump,
    payer = owner_authority, 
    space = 8 + std::mem::size_of::<NftListInfo>()
      )]
    pub nft_info_account: Account<'info, NftListInfo>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
#[derive(Accounts)]
pub struct RemoveItem<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
    mut,
     seeds = [constants::PDA_SEED],
     bump
    )]
    pub pda_account: SystemAccount<'info>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(
     mut,
     seeds = [constants::NFT_INFO_SEED, mint.key().as_ref()],
     bump,
      )]
    pub nft_info_account: Account<'info, NftListInfo>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
#[derive(Accounts)]
pub struct PurchaseItem<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub nft_owner: AccountInfo<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(
    mut,
     seeds = [constants::PDA_SEED],
     bump
    )]
    pub pda_account: SystemAccount<'info>,
    #[account(
     mut,
     seeds = [constants::NFT_INFO_SEED, mint.key().as_ref()],
     bump,
      )]
    pub nft_info_account: Account<'info, NftListInfo>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
#[account]
pub struct NftListInfo {
    pub created_at: u64,
    pub owner: Pubkey,
    pub price: u64,
    pub sold: bool,
    pub is_listed: bool,
    pub mint: Pubkey,
}
#[account]
pub struct CollectionListInfo {
    pub created_at: u64,
    pub owner: Pubkey,
    pub price: u64,
    pub sold: bool,
    pub is_listed: bool,
    pub mint: Pubkey,
}
#[error_code]
pub enum ErrorCode {
    #[msg("Wrong Mint Account Address.")]
    WrongMintAccount,
    #[msg("Must Be Owner Of NFT")]
    MustBeOwnerOfNFT,
    #[msg("Item Not Available")]
    NotAnNFT,
    #[msg("Item NFT not Listed")]
    NFTNotListed,
    #[msg("NFT Sold")]
    NFTSold,
    #[msg("Wrong Amount")]
    WrongAmount,
    #[msg("Not Avaiable")]
    NotAvaiable,
}
