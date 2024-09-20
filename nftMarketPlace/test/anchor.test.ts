import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
describe("NFT Marketplace", async () => {
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );
  const provider = await anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.NftMarket;

  it("list single NFT!", async () => {
    // Testing constants
    const saleAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
    const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
      "Hv2fsVxMvUEs1EfskftDJwEVsuKQYpEqXvibPqupkWgV"
    );
    const recipient = new anchor.web3.PublicKey(
      "529jaB5PAPBaoDjtyoD4y55K8B11jWcFY878iQtUtoyt"
    );
    // Derive the associated token address account for the mint and payer.
    // const senderTokenAddress = getAssociatedTokenAddressSync(
    //   mint,
    //   payer.publicKey
    // );

    const ownerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: payer.publicKey,
    });

    let [nftInfo] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("nft_info"), mint.toBuffer()],
      pg.PROGRAM_ID
    );
    let [RecipientAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), mint.toBuffer()],
      pg.PROGRAM_ID
    );
    // const metadataAddress = (
    //   await anchor.web3.PublicKey.findProgramAddressSync(
    //     [
    //       Buffer.from("metadata"), 
    //       TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    //       mint.toBuffer(),
    //     ],
    //     TOKEN_METADATA_PROGRAM_ID
    //   )
    // )[0];
    // console.log(`metadataAddress Address: ${metadataAddress}`);
    // console.log(`Request to sell NFT: ${mint} for ${saleAmount} lamports.`);
    console.log(`Owner's Token Address: ${ownerTokenAddress}`);
    // console.log(`recepient's Token Address: ${RecipientAccount.toString()}`);

    await program.methods
      .createItem(new BN(100))
      .accounts({
        mint: mint,
        ownerTokenAccount: ownerTokenAddress,
        ownerAuthority: pg.wallet.publicKey,
        // recipientAccount: RecipientAccount,
        recipientAccount: recipient,

        nftInfoAccount: nftInfo,
        // nftMetadata: metadataAddress,
        // metadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      // .signers([pg.wallet])
      .rpc({ skipPreflight: true });
  });

  // it("delist!", async () => {
  //   // Testing constants
  //   const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  //     "B6ZPVgLCU9v5M3GjEqERn8bjYHvz4MCSeZFtgmsXFLP3"
  //   );
  //   const recipient = new anchor.web3.PublicKey(
  //     "529jaB5PAPBaoDjtyoD4y55K8B11jWcFY878iQtUtoyt"
  //   );

  //   const ownerTokenAddress = await anchor.utils.token.associatedAddress({
  //     mint: mint,
  //     owner: pg.wallet.publicKey,
  //   });
  //   let [nftInfo] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [
  //       Buffer.from("nft_info"),
  //       mint.toBuffer(),
  //     ],
  //     pg.PROGRAM_ID
  //   );
  //   let [RecipientAccount] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("vault"), mint.toBuffer()],
  //     pg.PROGRAM_ID
  //   );
  //   console.log(`Owner's Token Address: ${ownerTokenAddress}`);
  //   console.log(`RecipientAccount's Token Address: ${RecipientAccount}`);
  //   await program.methods
  //     .removeItem()
  //     .accounts({
  //       mint: mint,
  //       signer: pg.wallet.publicKey,
  //       nftInfoAccount: nftInfo,
  //       ownerTokenAccount: ownerTokenAddress,
  //       recipientAccount: RecipientAccount,
  //     })
  //     // .signers([pg.wallet])
  //     .rpc({ skipPreflight: true });
  // });

  // it("purchase single NFT!", async () => {
  //   // Testing constants
  //   const saleAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
  //   // const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  //   //   "B6ZPVgLCU9v5M3GjEqERn8bjYHvz4MCSeZFtgmsXFLP3"
  //   // );
  //   //     const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  //   //   "Hv2fsVxMvUEs1EfskftDJwEVsuKQYpEqXvibPqupkWgV"
  //   // );
  //       const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  //     "5pTZWfoprGLHHdvrEFT6AheNtf5PrnBTu6XHeR3YcBjU"
  //   );
  //   const buyer = new anchor.web3.PublicKey(
  //     "5MjuAE8spr7DidyaYg6uNhaJ7Jo5FYmwWodjRKyZDfuk"
  //   );
  //   const buyerTokenAddress = await anchor.utils.token.associatedAddress({
  //     mint: mint,
  //     owner: buyer,
  //   });

  //   const nft_owner = new anchor.web3.PublicKey(
  //     "5MjuAE8spr7DidyaYg6uNhaJ7Jo5FYmwWodjRKyZDfuk"
  //   );
  //   // const ownerTokenAddress = await anchor.utils.token.associatedAddress({
  //   //   mint: mint,
  //   //   owner: payer.publicKey,
  //   // });

  //   let [nftInfo] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("nft_info"), mint.toBuffer()],
  //     pg.PROGRAM_ID
  //   );

  //   let [programAccount] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("vault"), mint.toBuffer()],
  //     pg.PROGRAM_ID
  //   );
  //   // const metadataAddress = (
  //   //   await anchor.web3.PublicKey.findProgramAddressSync(
  //   //     [
  //   //       Buffer.from("metadata"),
  //   //       TOKEN_METADATA_PROGRAM_ID.toBuffer(),
  //   //       mint.toBuffer(),
  //   //     ],
  //   //     TOKEN_METADATA_PROGRAM_ID
  //   //   )
  //   // )[0];
  //   // console.log(`metadataAddress Address: ${metadataAddress}`);
  //   // console.log(`Request to sell NFT: ${mint} for ${saleAmount} lamports.`);
  //   console.log(`Buyer's Token Address: ${buyerTokenAddress}`);

  //   await program.methods
  //     .purchaseItem(new BN(100))
  //     .accounts({
  //       signer: pg.wallet.publicKey,
  //       mint: mint,
  //       nftOwner: nft_owner,
  //       buyerTokenAccount: buyerTokenAddress,
  //       programNftAccount: programAccount,
  //       nftInfoAccount: nftInfo,
  //     })
  //     // .signers([pg.wallet])
  //     .rpc({ skipPreflight: true });
  // });
});
///////////////////////////

////////////////////////////////////////////////////////
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
