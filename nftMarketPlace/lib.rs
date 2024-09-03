use solana_program::clock::Clock;
use solana_program::system_instruction;
use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{
        associated_token,
        associated_token::{AssociatedToken, Create},
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3,
            mpl_token_metadata::accounts::Metadata as MD, mpl_token_metadata::types::DataV2,
            update_metadata_accounts_v2, CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata,
            UpdateMetadataAccountsV2,
        },
        token::spl_token::state::Account as AC,
        token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
    },
};

declare_id!("529jaB5PAPBaoDjtyoD4y55K8B11jWcFY878iQtUtoyt");
pub mod constants {
    pub const TOKEN_SEED: &[u8] = b"vault";
    pub const NFT_INFO_SEED: &[u8] = b"nft_info";

}
pub const PREFIX: &str = "metadata";

fn find_metadata_account(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            PREFIX.as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            mint.as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
}
#[program]
pub mod nft_market {
    use super::*;
    pub fn create_item(ctx: Context<CreateItem>, amount: u64) -> Result<()> {
        msg!("Transferring NFT...");
        let nft_info = &mut ctx.accounts.nft_info_account;
        // let md_account = find_metadata_account(&ctx.accounts.mint.key()).0;
        let data = MD::try_from(&ctx.accounts.nft_metadata.to_account_info());
        // let metadata_acc = MD::deserialize(&mut data.as_ref())?;
        msg!(" metadata_acc  {:?}", data);
        let owner_tk_acc = &ctx.accounts.owner_token_account;
        let user = &ctx.accounts.owner_authority;
        let nft_mint = &ctx.accounts.mint;

        //check if it is single nft
        if nft_mint.supply != 1 {
            return Err(ErrorCode::NotAnNFT.into());
        }

        if owner_tk_acc.amount != 1 {
            return Err(ErrorCode::NotAvaiable.into());
        }
        // //check if account belongs to mint
        if owner_tk_acc.mint != nft_mint.key() {
            return Err(ErrorCode::WrongMintAccount.into());
        }
        //check if user owns account
        if owner_tk_acc.owner != user.key() {
            return Err(ErrorCode::MustBeOwnerOfNFT.into());
        }
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.owner_token_account.to_account_info(),
                    to: ctx.accounts.recipient_account.to_account_info(),
                    authority: ctx.accounts.owner_authority.to_account_info(),
                },
            ),
            1,
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
        // check the owner of nft
        if nft_info.owner.key() != user.key() {
            return Err(ErrorCode::MustBeOwnerOfNFT.into());
        }
        // check if nft is listed
        if nft_info.is_listed != true {
            return Err(ErrorCode::NFTNotListed.into());
        }
        // check if nft is sole
        if nft_info.sold != false {
            return Err(ErrorCode::NFTSold.into());
        }
        // //check if account belongs to mint
        if owner_tk_acc.mint != mint.key() {
            return Err(ErrorCode::WrongMintAccount.into());
        }
        let bump = ctx.bumps.recipient_account;
        let signer: &[&[&[u8]]] =
            &[&[constants::TOKEN_SEED, user.as_ref(), mint.as_ref(), &[bump]]];
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.recipient_account.to_account_info(),
                    to: ctx.accounts.owner_token_account.to_account_info(),
                    authority: ctx.accounts.recipient_account.to_account_info(),
                },
                signer,
            ),
            1,
        )?;
        nft_info.is_listed = false;
        msg!("NFT transferred successfully.");
        Ok(())
    }

    pub fn purchase_item(ctx: Context<PurchaseItem>, amount: u64) -> Result<()> {
        msg!("Transferring amount...");
        let nft_info = &mut ctx.accounts.nft_info_account;
        let mint = ctx.accounts.mint.key();
        let owner = nft_info.owner;
        let receiver = &ctx.accounts.nft_owner;
        let buyer = &ctx.accounts.signer;
        if amount < nft_info.price {
            return Err(ErrorCode::WrongAmount.into());
        }
        // check if nft is listed
        if nft_info.is_listed == false {
            return Err(ErrorCode::NFTNotListed.into());
        }
        // check if nft is sole
        if nft_info.sold == true {
            return Err(ErrorCode::NFTSold.into());
        }

        msg!("Transferring NFT...");

        let bump = ctx.bumps.program_nft_account;
        let signer: &[&[&[u8]]] = &[&[
            constants::TOKEN_SEED,
            owner.as_ref(),
            mint.as_ref(),
            &[bump],
        ]];
        // transfer sol to owner

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
                    from: ctx.accounts.program_nft_account.to_account_info(),
                    to: ctx.accounts.buyer_token_account.to_account_info(),
                    authority: ctx.accounts.program_nft_account.to_account_info(),
                },
                signer,
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
        init_if_needed,
        seeds = [constants::TOKEN_SEED, owner_authority.key.as_ref(), mint.key().as_ref()],
        bump,
        payer = owner_authority, 
        token::mint = mint,
        token::authority = recipient_account   
    )]
    pub recipient_account: Account<'info, TokenAccount>,
    #[account(
     init_if_needed,
     seeds = [constants::NFT_INFO_SEED, owner_authority.key.as_ref(),mint.key().as_ref()],
     bump,
    payer = owner_authority, 
    space = 8 + std::mem::size_of::<NftListInfo>()
      )]
    pub nft_info_account: Account<'info, NftListInfo>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, Metadata>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub nft_metadata: UncheckedAccount<'info>,
}
#[derive(Accounts)]
pub struct RemoveItem<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
      )]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(
           mut,
        seeds = [constants::TOKEN_SEED, signer.key.as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub recipient_account: Account<'info, TokenAccount>,
    #[account(
     mut,
     seeds = [constants::NFT_INFO_SEED, signer.key.as_ref(), mint.key().as_ref()],
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
    #[account(mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
      )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [constants::TOKEN_SEED, nft_owner.key.as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub program_nft_account: Account<'info, TokenAccount>,

    #[account(
     mut,
     seeds = [constants::NFT_INFO_SEED, nft_owner.key.as_ref(), mint.key().as_ref()],
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
//new