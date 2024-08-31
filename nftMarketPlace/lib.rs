use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{
        associated_token,
        associated_token::{AssociatedToken, Create},
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3,
            mpl_token_metadata::types::DataV2, update_metadata_accounts_v2, CreateMasterEditionV3,
            CreateMetadataAccountsV3, Metadata, UpdateMetadataAccountsV2,
        },
        token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
    },
};

// use mpl_token_metadata::types::{Collection, Creator};
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

    // pub fn create_nft_item(
    //     ctx: Context<InitNFT>,
    //     name: String,
    //     symbol: String,
    //     uri: String,
    // ) -> Result<()> {
    //     update_metadata_accounts_v2(
    //         CpiContext::new(
    //             // CpiContext::new_with_signer(
    //             ctx.accounts.metadata_program.to_account_info(),
    //             UpdateMetadataAccountsV2 {
    //                 metadata: ctx.accounts.nft_metadata.to_account_info(),
    //                 update_authority: ctx.accounts.authority.to_account_info(),
    //             },
    //             // &[&seeds[..]],
    //         ),
    //         Some(ctx.accounts.authority.to_account_info().key()),
    //         Some(DataV2 {
    //             name,
    //             symbol,
    //             uri,
    //             seller_fee_basis_points: 0,
    //             creators: None,
    //             // creators: Some(vec![central_creator, CREATOR_FEE]),
    //             collection: None,
    //             uses: None,
    //         }),
    //         None,
    //         None,
    //     )?;

    //     // UpdateMetadataAccountsV2(cpi_context, data_v2, false, true, None)?;

    //     Ok(())
    // }
    // pub fn find_md(ctx: Context<FindItem>) -> Result<()> {
    //     let mint = ctx.accounts.mint.to_account_info();
    //     let metadata: Metadata = Metadata::from_a
    //     // :from_account_info(mint)?;
    //     msg!("mint account {:?}", metadata);
    //     Ok(())
    // }

    pub fn create_item(
        ctx: Context<CreateItem>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        msg!("Transferring NFT...");
        msg!(
            "Owner Token Address: {}",
            &ctx.accounts.owner_token_account.key()
        );
        msg!(
            " Token metadata: {}",
            find_metadata_account(&ctx.accounts.mint.key()).0
        );
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
        update_metadata_accounts_v2(
            CpiContext::new(
                // CpiContext::new_with_signer(
                ctx.accounts.metadata_program.to_account_info(),
                UpdateMetadataAccountsV2 {
                    metadata: ctx.accounts.nft_metadata.to_account_info(),
                    update_authority: ctx.accounts.owner_authority.to_account_info(),
                },
                // &[&seeds[..]],
            ),
            Some(ctx.accounts.recipient_account.to_account_info().key()),
            Some(DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points: 0,
                creators: None,
                // creators: Some(vec![central_creator, CREATOR_FEE]),
                collection: None,
                uses: None,
            }),
            None,
            None,
        )?;
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

#[derive(Accounts)]
pub struct InitNFT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
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
    //     #[account(
    //         mut,
    //         // address=find_metadata_account(&mint.key()).0,
    // )
    //     ]
    //     pub metadata_account: AccountInfo<'info>,
}
// #[derive(Accounts)]
// pub struct FindItem<'info> {
//     #[account(mut)]
//     pub mint: Account<'info, Mint>,
// }

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
// use {
//     anchor_lang::{prelude::*, solana_program::program::invoke, system_program},
//     anchor_spl::{associated_token, token},
//     mpl_token_auth_rules::instruction as token_instruction,
//     mpl_token_metadata::{instruction as token_instruction, ID as TOKEN_METADATA_ID},
// };
// declare_id!("529jaB5PAPBaoDjtyoD4y55K8B11jWcFY878iQtUtoyt");

// #[program]
// mod nft_market {
//     use super::*;
//     pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
//         ctx.accounts.new_account.data = data;
//         msg!("Changed data to: {}!", data); // Message will show up in the tx logs
//         Ok(())
//     }
//     // pub fn createItem(
//     //     ctx: Context<CreateItem>,
//     //     metadata_title: String,
//     //     metadata_symbol: String,
//     //     metadata_uri: String,
//     // ) -> Result<()> {
//     //     msg!("Creating mint account...");
//     //     msg!("Mint: {}", &ctx.accounts.mint.key());
//     //     system_program::create_account(
//     //         CpiContext::new(
//     //             ctx.accounts.token_program.to_account_info(),
//     //             system_program::CreateAccount {
//     //                 from: ctx.accounts.mint_authority.to_account_info(),
//     //                 to: ctx.accounts.mint.to_account_info(),
//     //             },
//     //         ),
//     //         10000000,
//     //         82,
//     //         &ctx.accounts.token_program.key(),
//     //     )?;

//     //     msg!("Initializing mint account...");
//     //     msg!("Mint: {}", &ctx.accounts.mint.key());
//     //     token::initialize_mint(
//     //         CpiContext::new(
//     //             ctx.accounts.token_program.to_account_info(),
//     //             token::InitializeMint {
//     //                 mint: ctx.accounts.mint.to_account_info(),
//     //                 rent: ctx.accounts.rent.to_account_info(),
//     //             },
//     //         ),
//     //         0,
//     //         &ctx.accounts.mint_authority.key(),
//     //         Some(&ctx.accounts.mint_authority.key()),
//     //     )?;

//     //     msg!("Creating token account...");
//     //     msg!("Token Address: {}", &ctx.accounts.token_account.key());
//     //     associated_token::create(CpiContext::new(
//     //         ctx.accounts.associated_token_program.to_account_info(),
//     //         associated_token::Create {
//     //             payer: ctx.accounts.mint_authority.to_account_info(),
//     //             associated_token: ctx.accounts.token_account.to_account_info(),
//     //             authority: ctx.accounts.mint_authority.to_account_info(),
//     //             mint: ctx.accounts.mint.to_account_info(),
//     //             system_program: ctx.accounts.system_program.to_account_info(),
//     //             token_program: ctx.accounts.token_program.to_account_info(),
//     //             rent: ctx.accounts.rent.to_account_info(),
//     //         },
//     //     ))?;

//     //     msg!("Minting token to token account...");
//     //     msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());
//     //     msg!("Token Address: {}", &ctx.accounts.token_account.key());
//     //     token::mint_to(
//     //         CpiContext::new(
//     //             ctx.accounts.token_program.to_account_info(),
//     //             token::MintTo {
//     //                 mint: ctx.accounts.mint.to_account_info(),
//     //                 to: ctx.accounts.token_account.to_account_info(),
//     //                 authority: ctx.accounts.mint_authority.to_account_info(),
//     //             },
//     //         ),
//     //         1,
//     //     )?;

//     //     msg!("Creating metadata account...");
//     //     msg!(
//     //         "Metadata account address: {}",
//     //         &ctx.accounts.metadata.to_account_info().key()
//     //     );
//     //     invoke(
//     //         &token_instruction::create_metadata_accounts_v2(
//     //             TOKEN_METADATA_ID,
//     //             ctx.accounts.metadata.key(),
//     //             ctx.accounts.mint.key(),
//     //             ctx.accounts.mint_authority.key(),
//     //             ctx.accounts.mint_authority.key(),
//     //             ctx.accounts.mint_authority.key(),
//     //             metadata_title,
//     //             metadata_symbol,
//     //             metadata_uri,
//     //             None,
//     //             1,
//     //             true,
//     //             false,
//     //             None,
//     //             None,
//     //         ),
//     //         &[
//     //             ctx.accounts.metadata.to_account_info(),
//     //             ctx.accounts.mint.to_account_info(),
//     //             ctx.accounts.token_account.to_account_info(),
//     //             ctx.accounts.mint_authority.to_account_info(),
//     //             ctx.accounts.rent.to_account_info(),
//     //         ],
//     //     )?;

//     //     msg!("Creating master edition metadata account...");
//     //     msg!(
//     //         "Master edition metadata account address: {}",
//     //         &ctx.accounts.master_edition.to_account_info().key()
//     //     );
//     //     invoke(
//     //         &token_instruction::create_master_edition_v3(
//     //             TOKEN_METADATA_ID,
//     //             ctx.accounts.master_edition.key(),
//     //             ctx.accounts.mint.key(),
//     //             ctx.accounts.mint_authority.key(),
//     //             ctx.accounts.mint_authority.key(),
//     //             ctx.accounts.metadata.key(),
//     //             ctx.accounts.mint_authority.key(),
//     //             Some(0),
//     //         ),
//     //         &[
//     //             ctx.accounts.master_edition.to_account_info(),
//     //             ctx.accounts.metadata.to_account_info(),
//     //             ctx.accounts.mint.to_account_info(),
//     //             ctx.accounts.token_account.to_account_info(),
//     //             ctx.accounts.mint_authority.to_account_info(),
//     //             ctx.accounts.rent.to_account_info(),
//     //         ],
//     //     )?;

//     //     msg!("Token mint process completed successfully.");

//     //     Ok(())
//     // }
//     pub fn init_nft(
//         ctx: Context<InitNFT>,
//         name: String,
//         symbol: String,
//         uri: String,
//     ) -> Result<()> {
//         // create mint account
//         let cpi_context = CpiContext::new(
//             ctx.accounts.token_program.to_account_info(),
//             MintTo {
//                 mint: ctx.accounts.mint.to_account_info(),
//                 to: ctx.accounts.associated_token_account.to_account_info(),
//                 authority: ctx.accounts.signer.to_account_info(),
//             },
//         );

//         mint_to(cpi_context, 1)?;

//         // create metadata account
//         let cpi_context = CpiContext::new(
//             ctx.accounts.token_metadata_program.to_account_info(),
//             CreateMetadataAccountsV3 {
//                 metadata: ctx.accounts.metadata_account.to_account_info(),
//                 mint: ctx.accounts.mint.to_account_info(),
//                 mint_authority: ctx.accounts.signer.to_account_info(),
//                 update_authority: ctx.accounts.signer.to_account_info(),
//                 payer: ctx.accounts.signer.to_account_info(),
//                 system_program: ctx.accounts.system_program.to_account_info(),
//                 rent: ctx.accounts.rent.to_account_info(),
//             },
//         );

//         let data_v2 = DataV2 {
//             name: name,
//             symbol: symbol,
//             uri: uri,
//             seller_fee_basis_points: 0,
//             creators: None,
//             collection: None,
//             uses: None,
//         };

//         create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

//         //create master edition account
//         let cpi_context = CpiContext::new(
//             ctx.accounts.token_metadata_program.to_account_info(),
//             CreateMasterEditionV3 {
//                 edition: ctx.accounts.master_edition_account.to_account_info(),
//                 mint: ctx.accounts.mint.to_account_info(),
//                 update_authority: ctx.accounts.signer.to_account_info(),
//                 mint_authority: ctx.accounts.signer.to_account_info(),
//                 payer: ctx.accounts.signer.to_account_info(),
//                 metadata: ctx.accounts.metadata_account.to_account_info(),
//                 token_program: ctx.accounts.token_program.to_account_info(),
//                 system_program: ctx.accounts.system_program.to_account_info(),
//                 rent: ctx.accounts.rent.to_account_info(),
//             },
//         );

//         create_master_edition_v3(cpi_context, None)?;

//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     // We must specify the space in order to initialize an account.
//     // First 8 bytes are default account discriminator,
//     // next 8 bytes come from NewAccount.data being type u64.
//     // (u64 = 64 bits unsigned integer = 8 bytes)
//     #[account(init, payer = signer, space = 8 + 8)]
//     pub new_account: Account<'info, NewAccount>,
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[account]
// pub struct NewAccount {
//     data: u64,
// }

// #[derive(Accounts)]
// pub struct CreateItem<'info> {
//     /// CHECK: We're about to create this with Metaplex
//     #[account(mut)]
//     pub metadata: UncheckedAccount<'info>,
//     /// CHECK: We're about to create this with Metaplex
//     #[account(mut)]
//     pub master_edition: UncheckedAccount<'info>,
//     #[account(mut)]
//     pub mint: Signer<'info>,
//     /// CHECK: We're about to create this with Anchor
//     #[account(mut)]
//     pub token_account: UncheckedAccount<'info>,
//     #[account(mut)]
//     pub mint_authority: Signer<'info>,
//     pub rent: Sysvar<'info, Rent>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
//     /// CHECK: Metaplex will check this
//     pub token_metadata_program: UncheckedAccount<'info>,
// }
// #[derive(Accounts)]
// pub struct InitNFT<'info> {
//     /// CHECK: ok, we are passing in this account ourselves
//     #[account(mut, signer)]
//     pub signer: AccountInfo<'info>,
//     #[account(
//         init,
//         payer = signer,
//         mint::decimals = 0,
//         mint::authority = signer.key(),
//         mint::freeze_authority = signer.key(),
//     )]
//     pub mint: Account<'info, Mint>,
//     #[account(
//         init_if_needed,
//         payer = signer,
//         associated_token::mint = mint,
//         associated_token::authority = signer
//     )]
//     pub associated_token_account: Account<'info, TokenAccount>,
//     /// CHECK - address
//     #[account(
//         mut,
//         address=find_metadata_account(&mint.key()).0,
//     )]
//     pub metadata_account: AccountInfo<'info>,
//     /// CHECK: address
//     #[account(
//         mut,
//         address=find_master_edition_account(&mint.key()).0,
//     )]
//     pub master_edition_account: AccountInfo<'info>,

//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub token_metadata_program: Program<'info, Metadata>,
//     pub system_program: Program<'info, System>,
//     pub rent: Sysvar<'info, Rent>,
// }
