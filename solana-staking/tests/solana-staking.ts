import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// import { SolanaStaking } from "../target/types/solana_staking";

// describe("solana-staking", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.SolanaStaking as Program<SolanaStaking>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });
//test written for playground
import {
  PublicKey,
  Keypair,
} from "@solana/web3.js";
import {
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
describe("Test", () => {
  // it("initialize", async () => {
  //   const provider = anchor.AnchorProvider.env();
  //   anchor.setProvider(provider);
  //   const payer = provider.wallet as anchor.Wallet;
  //   const mintKeyPair = Keypair.fromSecretKey(
  //     new Uint8Array([
  //       200, 9, 183, 147, 3, 125, 186, 181, 112, 130, 33, 156, 39, 141, 122,
  //       207, 186, 70, 158, 6, 53, 223, 124, 119, 183, 175, 93, 83, 62, 89, 235,
  //       136, 165, 204, 2, 221, 169, 106, 191, 24, 94, 215, 142, 208, 74, 149,
  //       120, 216, 178, 107, 184, 13, 200, 183, 58, 158, 86, 221, 248, 17, 179,
  //       112, 16, 160,
  //     ])
  //   );
  //   console.log("mintKeyPair", mintKeyPair.secretKey.toString());
  //   // async function createMintToken() {
  //   //   const mint = await createMint(
  //   //     pg.connection,
  //   //     payer.payer,
  //   //     payer.publicKey,
  //   //     payer.publicKey,
  //   //     9,
  //   //     mintKeyPair
  //   //   );
  //   //   console.log("mint", mint);
  //   // }
  //   // await createMintToken();
  //   let [vaultAccount] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("vault")],
  //     pg.PROGRAM_ID
  //   );
  //   // Send transaction

  //   const txHash = await pg.program.methods
  //     .initialize()
  //     .accounts({
  //       signer: pg.wallet.publicKey,
  //       tokenVaultAccount: vaultAccount,
  //       mint: mintKeyPair.publicKey,
  //     })
  //     .signers([payer.payer])
  //     .rpc();
  //   console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  // });
  // it("stake", async () => {
  //   const provider = anchor.AnchorProvider.env();
  //   anchor.setProvider(provider);
  //   const payer = provider.wallet as anchor.Wallet;
  //   const mintKeyPair = Keypair.fromSecretKey(
  //     new Uint8Array([
  //       200, 9, 183, 147, 3, 125, 186, 181, 112, 130, 33, 156, 39, 141, 122,
  //       207, 186, 70, 158, 6, 53, 223, 124, 119, 183, 175, 93, 83, 62, 89, 235,
  //       136, 165, 204, 2, 221, 169, 106, 191, 24, 94, 215, 142, 208, 74, 149,
  //       120, 216, 178, 107, 184, 13, 200, 183, 58, 158, 86, 221, 248, 17, 179,
  //       112, 16, 160,
  //     ])
  //   );
  //   // console.log("mintKeyPair", mintKeyPair.secretKey.toString());

  //   // let [vaultAccount] = PublicKey.findProgramAddressSync(
  //   //   [Buffer.from("vault")],
  //   //   pg.PROGRAM_ID
  //   // );
  //   let user_token_account = await getOrCreateAssociatedTokenAccount(
  //     pg.connection,
  //     payer.payer,
  //     mintKeyPair.publicKey,
  //     payer.publicKey
  //   );
  //   console.log("user_token_account", user_token_account.address.toString());

  //   let minting = await mintTo(
  //     pg.connection,
  //     payer.payer,
  //     mintKeyPair.publicKey,
  //     user_token_account.address,
  //     payer.payer,
  //     1e11
  //   );
  //   console.log("minting", minting);

  //   let [stakeInfo] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("stake_info"), payer.publicKey.toBuffer()],
  //     pg.PROGRAM_ID
  //   );
  //   console.log("stakeInfo", stakeInfo.toString());

  //   let [StakeAccount] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("token"), payer.publicKey.toBuffer()],
  //     pg.PROGRAM_ID
  //   );
  //   console.log("StakeAccount", StakeAccount.toString());
  //   let tokenAcc = await getOrCreateAssociatedTokenAccount(
  //     pg.connection,
  //     payer.payer,
  //     mintKeyPair.publicKey,
  //     payer.publicKey
  //   );
  //   console.log("tokenAcc", tokenAcc.address.toString());

  //   const txHash = await pg.program.methods
  //     .stake(new BN(100))
  //     .accounts({
  //       userTokenAccount: user_token_account.address,
  //       signer: pg.wallet.publicKey,
  //       stakeInfoAccount: stakeInfo,
  //       stakeAccount: StakeAccount,
  //       mint: mintKeyPair.publicKey,
  //     })
  //     .signers([payer.payer])
  //     .rpc();
  //   console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  // });
  it("destake", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const payer = provider.wallet as anchor.Wallet;
    const mintKeyPair = Keypair.fromSecretKey(
      new Uint8Array([
        200, 9, 183, 147, 3, 125, 186, 181, 112, 130, 33, 156, 39, 141, 122,
        207, 186, 70, 158, 6, 53, 223, 124, 119, 183, 175, 93, 83, 62, 89, 235,
        136, 165, 204, 2, 221, 169, 106, 191, 24, 94, 215, 142, 208, 74, 149,
        120, 216, 178, 107, 184, 13, 200, 183, 58, 158, 86, 221, 248, 17, 179,
        112, 16, 160,
      ])
    );
    // console.log("mintKeyPair", mintKeyPair.secretKey.toString());

    let [vaultAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      pg.PROGRAM_ID
    );
    console.log("vaultAccount", vaultAccount.toString());

    // let minting = await mintTo(
    //   pg.connection,
    //   payer.payer,
    //   mintKeyPair.publicKey,
    //   vaultAccount,
    //   payer.payer,
    //   1e11
    // );
    let user_token_account = await getOrCreateAssociatedTokenAccount(
      pg.connection,
      payer.payer,
      mintKeyPair.publicKey,
      payer.publicKey
    );
    console.log("user_token_account", user_token_account.address.toString());

    let [stakeInfo] = PublicKey.findProgramAddressSync(
      [Buffer.from("stake_info"), payer.publicKey.toBuffer()],
      pg.PROGRAM_ID
    );
    console.log("stakeInfo", stakeInfo.toString());

    let [StakeAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("token"), payer.publicKey.toBuffer()],
      pg.PROGRAM_ID
    );
    console.log("StakeAccount", StakeAccount.toString());
    let tokenAcc = await getOrCreateAssociatedTokenAccount(
      pg.connection,
      payer.payer,
      mintKeyPair.publicKey,
      payer.publicKey
    );
    console.log("tokenAcc", tokenAcc.address.toString());

    const txHash = await pg.program.methods
      .destake()
      .accounts({
        userTokenAccount: user_token_account.address,
        signer: pg.wallet.publicKey,
        stakeInfoAccount: stakeInfo,
        stakeAccount: StakeAccount,
        mint: mintKeyPair.publicKey,
        tokenVaultAccount: vaultAccount,
      })
      .signers([payer.payer])
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  });
});
