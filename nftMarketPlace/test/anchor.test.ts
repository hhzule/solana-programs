import { getAssociatedTokenAddressSync } from "@solana/spl-token";
// import { Keypair } from "@solana/web3.js";
// // import type { NftMinter } from "../target/types/nft_minter";

describe("NFT Minter", () => {
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.NftMarket;
  // as anchor.Program<NftMinter>;

  // it("Create an NFT!", async (done) => {
  //   // Generate a keypair to use as the address of our mint account
  //   const mintKeypair = anchor.web3.Keypair.generate();
  //   console.log("mintKeypair initialized", mintKeypair.publicKey.toString());
  //   const metadataAddress = (
  //     await anchor.web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.from("metadata"),
  //         TOKEN_METADATA_PROGRAM_ID.toBuffer(),
  //         mintKeypair.publicKey.toBuffer(),
  //       ],
  //       TOKEN_METADATA_PROGRAM_ID
  //     )
  //   )[0];
  //   console.log("Metadata initialized", metadataAddress.toString());
  //   const masterEditionAddress = (
  //     await anchor.web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.from("metadata"),
  //         TOKEN_METADATA_PROGRAM_ID.toBuffer(),
  //         mintKeypair.publicKey.toBuffer(),
  //         Buffer.from("edition"),
  //       ],
  //       TOKEN_METADATA_PROGRAM_ID
  //     )
  //   )[0];
  //   console.log(
  //     "Master edition metadata initialized",
  //     masterEditionAddress.toString()
  //   );

  //   // // Derive the associated token address account for the mint_authority/payer/owner.
  //   const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
  //     mintKeypair.publicKey,
  //     payer.publicKey
  //   );
  //   console.log(
  //     "associatedTokenAccountAddress",
  //     associatedTokenAccountAddress.toString()
  //   );
  //   // // Derive the associated token address account for the program.
  //   const associatedTokenAccountAddressProgram = getAssociatedTokenAddressSync(
  //     mintKeypair.publicKey,
  //     payer.publicKey
  //   );
  //   console.log(
  //     "associatedTokenAccountAddress",
  //     associatedTokenAccountAddress.toString()
  //   );
  //   const transactionSignature = await program.methods
  //     .createNftItem(metadata.name, metadata.symbol, metadata.uri)
  //     .accounts({
  //       // signer: pg.wallet,
  //       signer: mintKeypair,
  //       mint: mintKeypair.publicKey,
  //       associatedTokenAccount: associatedTokenAccountAddress,
  //       masterEditionAccount: masterEditionAddress,
  //       metadataAccount: metadataAddress,
  //       tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,

  //       //       tokenProgram: pg.TOKEN_PROGRAM_ID,
  //       // associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       systemProgram: web3.SystemProgram.programId,
  //       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //     })
  //     .signers([mintKeypair])
  //     .rpc({ skipPreflight: true });
  //   done();
  //   console.log("Success!");
  //   console.log(`   Mint Address: ${mintKeypair.publicKey}`);
  //   console.log(`   Transaction Signature: ${transactionSignature}`);
  // }).timeout(10000);
  it("list!", async () => {
    // Testing constants

    const saleAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
    const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
      "B6ZPVgLCU9v5M3GjEqERn8bjYHvz4MCSeZFtgmsXFLP3"
    );
    const recipient = new anchor.web3.PublicKey(
      "529jaB5PAPBaoDjtyoD4y55K8B11jWcFY878iQtUtoyt"
    );
    // Derive the associated token address account for the mint and payer.
    const senderTokenAddress = getAssociatedTokenAddressSync(
      mint,
      payer.publicKey
    );

    // Derive the associated token address account for the mint and recipient.
    const recepientTokenAddress = getAssociatedTokenAddressSync(
      mint,
      recipient
    );

    const ownerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: payer.publicKey,
    });
    const buyerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: recipient,
    });
    let [stakeInfo] = PublicKey.findProgramAddressSync(
      [Buffer.from("stake_info"), payer.publicKey.toBuffer()],
      pg.PROGRAM_ID
    );
    let [StakeAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), payer.publicKey.toBuffer(), mint.toBuffer()],
      pg.PROGRAM_ID
    );
    const metadataAddress = (
      await anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
    console.log(`Request to sell NFT: ${mint} for ${saleAmount} lamports.`);
    console.log(`Owner's Token Address: ${ownerTokenAddress}`);
    console.log(`Buyer's Token Address: ${buyerTokenAddress}`);

    await program.methods
      .createItem(
        "ZH",
        "ZH",
        "https://arweave.net/mYoq2DmUvil9Kw9lCzUzI75Q-uJ4BDVgqvPL4iw-GhQ"
      )
      .accounts({
        mint: mint,
        ownerTokenAccount: ownerTokenAddress,
        ownerAuthority: pg.wallet.publicKey,
        recipientAccount: StakeAccount,
        stakeInfoAccount: stakeInfo,
        nftMetadata: metadataAddress,
        metadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      // .signers([pg.wallet])
      .rpc({ skipPreflight: true });
  });

  // #[account(
  //     mut,
  //     seeds = [
  //         b"metadata".as_ref(),
  //         metadata_program.key().as_ref(),
  //         mint.key().as_ref(),
  //     ],
  //     bump,
  //     seeds::program = metadata_program.key()
  // )]
  // pub nft_metadata: UncheckedAccount<'info>,
  // it("delist!", async () => {
  //   // Testing constants

  //   const saleAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
  //   const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  //     "B6ZPVgLCU9v5M3GjEqERn8bjYHvz4MCSeZFtgmsXFLP3"
  //   );
  //   const recipient = new anchor.web3.PublicKey(
  //     "529jaB5PAPBaoDjtyoD4y55K8B11jWcFY878iQtUtoyt"
  //   );

  //   const ownerTokenAddress = await anchor.utils.token.associatedAddress({
  //     mint: mint,
  //     owner: payer.publicKey,
  //   });

  //   let [StakeAccount] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("vault"), payer.publicKey.toBuffer(), mint.toBuffer()],
  //     pg.PROGRAM_ID
  //   );
  //   console.log(`Request to sell NFT: ${mint} for ${saleAmount} lamports.`);
  //   console.log(`Owner's Token Address: ${ownerTokenAddress}`);

  //   await program.methods
  //     .removeItem()
  //     .accounts({
  //       mint: mint,
  //       signer: pg.wallet.publicKey,
  //       ownerTokenAccount: ownerTokenAddress,
  //       recipientAccount: StakeAccount,
  //     })
  //     // .signers([pg.wallet])
  //     .rpc({ skipPreflight: true });
  // });
});
