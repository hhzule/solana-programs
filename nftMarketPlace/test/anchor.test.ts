import {
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

describe("NFT Marketplace", async () => {
  const provider = await anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.NftMarket;

  // it("initialize!", async () => {
  //   const lamports = anchor.web3.LAMPORTS_PER_SOL;
  //   let [factory_config] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("factory_config")],
  //     pg.PROGRAM_ID
  //   );
  //   console.log(`nftInfo Address: ${factory_config}`);
  //   const fee_collector_info = new anchor.web3.PublicKey(
  //     "HpVdgHTUUmCJxc8npDB7YwUPaWpMaccxBBHc13TZqR9B"
  //   );
  //   await program.methods
  //     .initialize(2)
  //     .accounts({
  //       admin: pg.wallet.publicKey,
  //       factoryConfig: factory_config,
  //       feeCollectorInfo: fee_collector_info,
  //     })
  //     .rpc({ skipPreflight: true });
  // });
  it("list single NFT!", async () => {
    const lamports = anchor.web3.LAMPORTS_PER_SOL;
    const saleAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
    const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
      "CJCWa4fiYzgKm8vtkSTwPuELqPHdJaU9yHCHsC1Lb9EP"
    );
    const ownerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: payer.publicKey,
    });
    console.log(`Owner's Token Address: ${ownerTokenAddress}`);
    let [nftInfo] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("nft_info"), mint.toBuffer()],
      pg.PROGRAM_ID
    );
    console.log(`nftInfo Address: ${nftInfo}`);
    let [pdaAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace_pda_seed")],
      pg.PROGRAM_ID
    );
    console.log(`pdaAccount Address: ${pdaAccount}`);
    let [factory_config] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("factory_config")],
      pg.PROGRAM_ID
    );
    console.log(`nftInfo Address: ${factory_config}`);
    const fee_collector_info = new anchor.web3.PublicKey(
      "HpVdgHTUUmCJxc8npDB7YwUPaWpMaccxBBHc13TZqR9B"
    );
    await program.methods
      .createItem(new BN(1))
      .accounts({
        mint: mint,
        ownerTokenAccount: ownerTokenAddress,
        ownerAuthority: pg.wallet.publicKey,
        pdaAccount: pdaAccount,
        nftInfoAccount: nftInfo,
        factoryConfig: factory_config,
        feeCollector: fee_collector_info,
      })
      .rpc({ skipPreflight: true });
  });

  // it("delist!", async () => {
  //   const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  //     "CJCWa4fiYzgKm8vtkSTwPuELqPHdJaU9yHCHsC1Lb9EP"
  //   );
  //   const ownerTokenAddress = await anchor.utils.token.associatedAddress({
  //     mint: mint,
  //     owner: pg.wallet.publicKey,
  //   });
  //   let [nftInfo] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("nft_info"), mint.toBuffer()],
  //     pg.PROGRAM_ID
  //   );
  //   console.log(`Owner's Token Address: ${ownerTokenAddress}`);
  //   let [pdaAccount] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("marketplace_pda_seed")],
  //     pg.PROGRAM_ID
  //   );
  //   await program.methods
  //     .removeItem()
  //     .accounts({
  //       mint: mint,
  //       signer: pg.wallet.publicKey,
  //       nftInfoAccount: nftInfo,
  //       ownerTokenAccount: ownerTokenAddress,
  //       pdaAccount: pdaAccount,
  //     })
  //     .rpc({ skipPreflight: true });
  // });

  it("purchase single NFT!", async () => {
    const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
      "2EN7hgfk3nXehjwYKekhsQvcyL73BuEiBYQ2b6x6yJK8"
    );
    const buyer = new anchor.web3.PublicKey(
      "HpVdgHTUUmCJxc8npDB7YwUPaWpMaccxBBHc13TZqR9B"
    );
    let tokenAcc = await getOrCreateAssociatedTokenAccount(
      pg.connection,
      payer.payer,
      mint,
      buyer
    );
    console.log(`tokenAcc Address: ${tokenAcc.address}`);
    const buyerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: buyer,
    });
    console.log(`Buyer's Token Address: ${buyerTokenAddress}`);
    const nft_owner = new anchor.web3.PublicKey(
      "5MjuAE8spr7DidyaYg6uNhaJ7Jo5FYmwWodjRKyZDfuk"
    );
    const ownerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: nft_owner,
    });
    console.log(`ownerTokenAddress Address: ${ownerTokenAddress}`);

    let [nftInfo] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("nft_info"), mint.toBuffer()],
      pg.PROGRAM_ID
    );
    let [pdaAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace_pda_seed")],
      pg.PROGRAM_ID
    );
    console.log(`pdaAccount Address: ${pdaAccount}`);
    let [factory_config] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("factory_config")],
      pg.PROGRAM_ID
    );
    console.log(`nftInfo Address: ${factory_config}`);
    const fee_collector_info = new anchor.web3.PublicKey(
      "HpVdgHTUUmCJxc8npDB7YwUPaWpMaccxBBHc13TZqR9B"
    );
    await program.methods
      .purchaseItem(new BN(2))
      .accounts({
        signer: pg.wallet.publicKey,
        mint: mint,
        nftOwner: nft_owner,
        buyerTokenAccount: buyerTokenAddress,
        pdaAccount: pdaAccount,
        nftInfoAccount: nftInfo,
        ownerTokenAccount: ownerTokenAddress,
        factoryConfig: factory_config,
        feeCollector: fee_collector_info,
      })

      // .signers([payer.payer])
      .rpc({ skipPreflight: true });
  });
});
///////////////////////////
