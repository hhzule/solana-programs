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
        token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
    },
};

declare_id!("529jaB5PAPBaoDjtyoD4y55K8B11jWcFY878iQtUtoyt");
pub mod constants {
    pub const TOKEN_SEED: &[u8] = b"vault";
    pub coNFT: &[u8] = b"stake_info";

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

    pub fn create_item(
        ctx: Context<CreateItem>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        msg!("Transferring NFT...");
        let md_account = find_metadata_account(&ctx.accounts.mint.key()).0;
        let data = MD::try_from(&ctx.accounts.nft_metadata.to_account_info());
        let metadata_acc = MD::deserialize(&mut data.as_ref());
        msg!(" metadata_acc  {:?}", metadata_acc);
        let owner_tk_acc = &ctx.accounts.owner_token_account;
        let user = &ctx.accounts.owner_authority;
        let nft_mint = &ctx.accounts.mint;

        // check the owner of nft
        assert_eq!(owner_tk_acc.owner, user.key());
        //check the mint of nft
        assert_eq!(owner_tk_acc.mint, nft_mint.key());
        //check the amount of nft
        assert_eq!(owner_tk_acc.amount, 1);
        //check if metadata account belongs to mint
        assert_eq!(&ctx.accounts.nft_metadata.key(), md_account);

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
        msg!("NFT transferred successfully.");

        Ok(())
    }
    pub fn remove_item(ctx: Context<RemoveItem>) -> Result<()> {
        msg!("Transferring NFT...");
        let mint = ctx.accounts.mint.key();
        let staker = ctx.accounts.signer.key();
        let bump = ctx.bumps.recipient_account;
        let signer: &[&[&[u8]]] = &[&[
            constants::TOKEN_SEED,
            staker.as_ref(),
            mint.as_ref(),
            &[bump],
        ]];
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
        msg!("NFT transferred successfully.");
        Ok(())
    }
  pub fn purchase_item(ctx: Context<PurchaseItem>, _amount : u64) -> Result<()> {
    msg!("Transferring amount...");

        msg!("Transferring NFT...");
        let mint = ctx.accounts.mint.key();
        let staker = ctx.accounts.signer.key();
        let bump = ctx.bumps.recipient_account;
        let signer: &[&[&[u8]]] = &[&[
            constants::TOKEN_SEED,
            staker.as_ref(),
            mint.as_ref(),
            &[bump],
        ]];
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
    // #[account(mut)]
    // pub update_authority: Signer<'info>,
    #[account(mut,
        associated_token::mint = mint,
        associated_token::authority = owner_authority,
      )]
    pub owner_token_account: Account<'info, TokenAccount>,
    pub recipient: SystemAccount<'info>,
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
     seeds = [constants::NFT_INFO_SEED, owner_authority.key.as_ref()],
     bump,
    payer = owner_authority, 
    space = 8 + std::mem::size_of::<NftListInfo>()
      )]
    pub nft_info_account: Account<'info, NftListInfo>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    // #[account(
    //     init_if_needed,
    //     payer = owner_authority,
    //     associated_token::mint = mint,
    //     associated_token::authority = owner_authority
    // )]
    // pub token_account: Account<'info, TokenAccount>,
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
     seeds = [constants::NFT_INFO_SEED, owner_authority.key.as_ref()],
     bump,
    payer = owner_authority, 
    space = 8 + std::mem::size_of::<NftListInfo>()
      )]
    pub nft_info_account: Account<'info, NftListInfo>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
}

#[account]
pub struct NftListInfo {
    // pub current_nfts: Vec::new(),
}
