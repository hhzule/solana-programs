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
    pub const STAKE_INFO_SEED: &[u8] = b"stake_info";

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
        // let data = MD::try_from(&ctx.accounts.nft_metadata.to_account_info());
        // let owner_tk_acc = &ctx.accounts.owner_token_account;
        let user = &ctx.accounts.owner_authority;
        let nft_mint = &ctx.accounts.mint;
        // let metadata = &ctx.accounts.nft_metadata;
        // msg!(" metadata {:?}", metadata);
        //check the owner of nft
        // assert_eq!(owner_tk_acc.owner, user.key());
        //check the mint of nft
        // assert_eq!(owner_tk_acc.mint, nft_mint.key());
        //check the amount of nft
        // assert_eq!(owner_tk_acc.amount, 1);
        //check if metadata account belongs to mint
        // assert_eq!(&ctx.accounts.nft_metadata.key(), md_account);
        // let data = ctx.accounts.nft_metadata.data.try_borrow_mut().unwrap();

        // let metadata_acc = MD::deserialize(&mut data.as_ref());
        // msg!(" metadata_acc  {:?}", metadata_acc);
        // Deserialize the metadata account
        // Deserialize the metadata account
        let nft_metadata = &ctx.accounts.nft_metadata;
        let data = nft_metadata.try_borrow_data()?;
        let metadata = MD::try_from_slice(&data)?;

        msg!("Metadata: {:?}", metadata);

        // Access individual fields directly
        msg!("Update Authority: {:?}", metadata.update_authority);
        msg!("Mint: {:?}", metadata.mint);
        msg!("Name: {:?}", metadata.name);
        msg!("Symbol: {:?}", metadata.symbol);
        msg!("URI: {:?}", metadata.uri);

        // let result: Result<Metadata, std::io::Error> = metadata_acc;
        // match result {
        //     Ok(metadata) => {
        //         // Access the `update_authority` field
        //         println!("Update Authority: {}", metadata);
        //     }
        //     Err(e) => {
        //         println!("Error occurred: {}", e);
        //     }
        // }
        // if data.data_is_empty() {
        //     msg!(" metadata empty");
        // } else {
        //     msg!(" metadata exists");
        // }
        // if let Some(collection) = &data.collection {
        //     if collection.verified {
        //         return Ok(());
        //     }
        // }

        // let metadata: Metadata =
        //     Metadata::from_account_info(&ctx.accounts.nft_collection_metadata.to_account_info())?;
        //  Ok(Metadata { key: MetadataV1,
        // update_authority: HpVdgHTUUmCJxc8npDB7YwUPaWpMaccxBBHc13TZqR9B,
        // mint: B6ZPVgLCU9v5M3GjEqERn8bjYHvz4MCSeZFtgmsXFLP3,
        // name: "ZuleHanif",
        // symbol: "ZH\0\0\0\0\0\0\0\0",
        // uri: "https://arweave.net/mYoq2DmUvil9Kw9lCzUzI75Q-uJ4BDVgqvPL4iw-GhQ",
        //  seller_fee_basis_points: 0,
        // creators: Some([Creator { address: HpVdgHTUUmCJxc8npDB7YwUPaWpMaccxBBHc13TZqR9B, verified: true, share: 100 }]),
        // primary_sale_happened: false,
        // is_mutable: true,
        // edition_nonce: Some(254),
        // token_standard: Some(Fungible),
        // collection: None,
        // uses: None,
        // collection_details: None,
        // programmable_config: None })

        // transfer(
        //     CpiContext::new(
        //         ctx.accounts.token_program.to_account_info(),
        //         Transfer {
        //             from: ctx.accounts.owner_token_account.to_account_info(),
        //             to: ctx.accounts.recipient_account.to_account_info(),
        //             authority: ctx.accounts.owner_authority.to_account_info(),
        //         },
        //     ),
        //     1,
        // )?;
        // msg!("NFT transferred successfully.");
        // update_metadata_accounts_v2(
        //     CpiContext::new(
        //         // CpiContext::new_with_signer(
        //         ctx.accounts.metadata_program.to_account_info(),
        //         UpdateMetadataAccountsV2 {
        //             metadata: ctx.accounts.nft_metadata.to_account_info(),
        //             update_authority: ctx.accounts.owner_authority.to_account_info(),
        //         },
        //         // &[&seeds[..]],
        //     ),
        //     Some(ctx.accounts.recipient_account.to_account_info().key()),
        //     Some(DataV2 {
        //         name: data.name(),
        //         symbol: data.symbol,
        //         uri,
        //         seller_fee_basis_points: 0,
        //         creators: None,
        //         // creators: Some(vec![central_creator, CREATOR_FEE]),
        //         collection: None,
        //         uses: None,
        //     }),
        //     None,
        //     None,
        // )?;
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

}

// #[derive(Accounts)]
// pub struct InitNFT<'info> {
//     #[account(
//         init_if_needed,
//         payer = authority,
//         associated_token::mint = mint,
//         associated_token::authority = authority
//     )]
//     pub token_account: Account<'info, TokenAccount>,
//     pub metadata_program: Program<'info, Metadata>,
//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     pub nft_metadata: UncheckedAccount<'info>,
//     //     #[account(
//     //         mut,
//     //         // address=find_metadata_account(&mint.key()).0,
//     // )
//     //     ]
//     //     pub metadata_account: AccountInfo<'info>,
// }

#[derive(Accounts)]
pub struct CreateItem<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    // #[account(mut)]
    // pub update_authority: Signer<'info>,
    // #[account(mut,
    //     associated_token::mint = mint,
    //     associated_token::authority = owner_authority,
    //   )]
    // pub owner_token_account: Account<'info, TokenAccount>,
    // pub recipient: SystemAccount<'info>,
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
     seeds = [constants::STAKE_INFO_SEED, owner_authority.key.as_ref()],
     bump,
    payer = owner_authority, 
    space = 8 + std::mem::size_of::<StakeInfo>()
      )]
    pub stake_info_account: Account<'info, StakeInfo>,
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
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
}

#[account]
pub struct StakeInfo {
    // pub current_nfts: Vec::new(),
}
