import {
  workspace,
  AnchorProvider,
  getProvider,
  setProvider,
  Wallet,
  BN,
  Program,
} from "@coral-xyz/anchor";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  LAMPORTS_PER_SOL,
  SYSVAR_RENT_PUBKEY,
} from '@solana/web3.js'
import {
  getOrCreateAssociatedTokenAccount,
  createMint,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

import { assert } from "chai";
import { createRandomMint, createRandomWalletAndAirdrop, getRandomNumber, programPaidBy, waitSeconds } from "./utils";
import { provider, program } from './config';

describe("cre-ico-solana", () => {
  // Configure the client to use the local cluster.
  setProvider(provider);

  // @ts-ignore
  let admin = getProvider().wallet;

  // These are all of the variables we assume exist in the world already and are available to the client.

  const ONE_USDT = 1000000

  let protocol_wallet = null;
  let user1 = null;
  let user_program = null;
  let ico_token_mint_decimals = null;
  let ico_token_mint = null;
  let admin_ico_token_account = null;
  let usdc_mint = null, usdt_mint = null;
  let user_usdt_token_account = null, user_usdc_token_account = null;

  before(async () => {
    protocol_wallet = await createRandomWalletAndAirdrop(provider, 2);
    user1 = await createRandomWalletAndAirdrop(provider, 2);

    ico_token_mint_decimals = 9;
    ico_token_mint = await createRandomMint(provider, ico_token_mint_decimals);

    console.log('\nICO Token Mint: ', ico_token_mint.toString())
    console.log('ICO Token Decimals: ', 9)

    admin_ico_token_account = await getOrCreateAssociatedTokenAccount(provider.connection, admin.payer, ico_token_mint, admin.publicKey);

    const total_ico_amount = 20000
    const total_ico_amount_lamports = new BN(10 ** ico_token_mint_decimals * total_ico_amount)

    await mintTo(
      provider.connection,
      admin.payer,
      ico_token_mint,
      admin_ico_token_account.address,
      admin.publicKey,
      BigInt(total_ico_amount_lamports.toNumber())
    );

    const admin_ico_account_balance_info = await provider.connection.getTokenAccountBalance(admin_ico_token_account.address)
    assert.equal(admin_ico_account_balance_info.value.amount.toString(), total_ico_amount_lamports.toString(), "Ico token should be minted to admin wallet");
    console.log("Admin ICO Token Account Balance: ", admin_ico_account_balance_info.value.amount)

    // Create mock stable token mints and mint to user wallet
    usdt_mint = await createRandomMint(provider, 6);
    usdc_mint = await createRandomMint(provider, 6);

    user_usdt_token_account = await getOrCreateAssociatedTokenAccount(provider.connection, admin.payer, usdt_mint, user1.publicKey);
    user_usdc_token_account = await getOrCreateAssociatedTokenAccount(provider.connection, admin.payer, usdc_mint, user1.publicKey);

    const user_usdt = 1000
    const user_usdt_amount = new BN(10 ** 6 * user_usdt)

    await mintTo(
      provider.connection,
      admin.payer,
      usdt_mint,
      user_usdt_token_account.address,
      admin.publicKey,
      BigInt(user_usdt_amount.toNumber())
    );
    await mintTo(
      provider.connection,
      admin.payer,
      usdc_mint,
      user_usdc_token_account.address,
      admin.publicKey,
      BigInt(user_usdt_amount.toNumber())
    );

    const user_usdt_token_account_balance_info = await provider.connection.getTokenAccountBalance(user_usdt_token_account.address)
    assert.equal(user_usdt_token_account_balance_info.value.amount.toString(), user_usdt_amount.toString(), "Mock USDT token should be minted to user wallet");
    console.log("User USDT Token Account Balance: ", user_usdt_token_account_balance_info.value.amount)

    const user_usdc_token_account_balance_info = await provider.connection.getTokenAccountBalance(user_usdc_token_account.address)
    assert.equal(user_usdc_token_account_balance_info.value.amount.toString(), user_usdt_amount.toString(), "Mock USDC token should be minted to user wallet");
    console.log("User USDC Token Account Balance: ", user_usdc_token_account_balance_info.value.amount)
    console.log('\n');
  });

  // These are all variables the client will need to create in order to initialize the ICO pool
  let icoName = "cre_ico";

  it('Initializes the ICO', async () => {
    console.log("---Initializes the presale---\n")

    const protocol_ico_amount = 15000
    const protocol_ico_amount_lamports = new BN(10 ** ico_token_mint_decimals * protocol_ico_amount)
    const token_per_usd = new BN(5) // ico token 1 USDT

    // Fetch the PDA of ico info account
    const [ico_info_pda, ico_info_bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from(icoName)],
      program.programId
    );

    const [ico_state_pda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("ico_state")],
      program.programId
    );

    const [protocol_ico_token_pda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("protocol_ico_token")],
      program.programId
    );

    const [protocol_usdt_pool_pda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("protocol_usdt_pool")],
      program.programId
    );

    await program.methods
      .initialize(
        icoName,
        protocol_ico_amount_lamports,
        token_per_usd,
        ico_info_bump
      )
      .accounts({
        icoInfo: ico_info_pda,
        icoState: ico_state_pda,
        authority: admin.publicKey,
        adminIcoTokenAccount: admin_ico_token_account.address,
        protocolIcoTokenPda: protocol_ico_token_pda,
        // protocolSolPoolPda: protocol_sol_pool_pda,
        protocolUsdtPoolPda: protocol_usdt_pool_pda,
        icoTokenMint: ico_token_mint,
        usdcMint: usdc_mint,
        usdtMint: usdt_mint,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .rpc();

    const protocol_ico_account_balance_info = await provider.connection.getTokenAccountBalance(protocol_ico_token_pda)
    assert.equal(protocol_ico_account_balance_info.value.amount.toString(), protocol_ico_amount_lamports.toNumber().toString(), "Ico token should be transferred to protocol token account");
    console.log("Protocol ICO Token Lamports: ", protocol_ico_amount_lamports.toNumber().toString())

    const ico_info = await program.account.icoInfo.fetch(ico_info_pda);
    console.log('ICO Authority: ', ico_info.authority.toString());
    // console.log('ICO Protocol Wallet: ', ico_info.protocolWallet.toString());
    console.log('ICO Amount: ', ico_info.totalIcoAmount.toString());
    console.log('ICO Token Per SOL: ', ico_info.tokenPerUsd.toString());
    const ico_state = await program.account.icoState.fetch(ico_state_pda);
    console.log('ICO Remaining: ', ico_state.remainingIcoAmount.toString());
    console.log('ICO Total SOL: ', ico_state.totalSoldUsd.toString());
    console.log("\n");
  });

  it("Method: depositUSDT", async function () {
    const depositUSDT = new BN(1.5 * ONE_USDT);

    // Fetch the PDA of ICO info account
    const [ico_info_pda] = await PublicKey.findProgramAddressSync(
      [Buffer.from(icoName)],
      program.programId
    );

    const [ico_state_pda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("ico_state")],
      program.programId
    );

    user_usdt_token_account = await getOrCreateAssociatedTokenAccount(provider.connection, admin.payer, usdt_mint, user1.publicKey);

    const [protocol_usdt_pool_pda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("protocol_usdt_pool")],
      program.programId
    );

    // create ICO token account for user wallet
    const user_ico_token_account = await getOrCreateAssociatedTokenAccount(provider.connection, admin.payer, ico_token_mint, user1.publicKey);

    const [protocol_ico_token_pda, protocol_ico_token_bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("protocol_ico_token")],
      program.programId
    );

    await program.methods
      .depositUsdt(depositUSDT)
      .accounts({
        icoInfo: ico_info_pda,
        icoState: ico_state_pda,
        user: user1.publicKey,
        userUsdtTokenAccount: user_usdt_token_account.address,
        protocolUsdtPoolPda: protocol_usdt_pool_pda,
        userIcoTokenAccount: user_ico_token_account.address,
        protocolIcoTokenPda: protocol_ico_token_pda,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([user1])
      .rpc();

    const ico_info = await program.account.icoInfo.fetch(ico_info_pda);

    const ico_state = await program.account.icoState.fetch(ico_state_pda);
    console.log('ICO Remaining: ', ico_state.remainingIcoAmount.toString());
    console.log('Total Sold Amount in USD: ', ico_state.totalSoldUsd.toString());
    console.log('Total USDT in pool: ', ico_state.totalUsdt.toString());
    console.log("\n");
    const protocol_usdt_pool_balance_info = await provider.connection.getTokenAccountBalance(protocol_usdt_pool_pda)
    assert.equal(protocol_usdt_pool_balance_info.value.amount.toString(), ico_state.totalUsdt.toString(), "USDT should have deposited to protocol usdt pool");

    const user_ico_token_balance_info = await provider.connection.getTokenAccountBalance(user_ico_token_account.address)
    assert.equal(user_ico_token_balance_info.value.amount.toString(), (ico_info.totalIcoAmount.toNumber() - ico_state.remainingIcoAmount.toNumber()).toString(), "USDT should have deposited to protocol usdt pool");
  });

  it("Method: withdrawUSDT", async function () {
    const depositUSDT = new BN(1.5 * ONE_USDT);

    // Fetch the PDA of ICO info account
    const [ico_info_pda] = await PublicKey.findProgramAddressSync(
      [Buffer.from(icoName)],
      program.programId
    );

    const [ico_state_pda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("ico_state")],
      program.programId
    );

    const admin_usdt_token_account = await getOrCreateAssociatedTokenAccount(provider.connection, admin.payer, usdt_mint, admin.publicKey);

    const [protocol_usdt_pool_pda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("protocol_usdt_pool")],
      program.programId
    );

    await program.methods
      .withdrawUsdt()
      .accounts({
        icoInfo: ico_info_pda,
        icoState: ico_state_pda,
        authority: admin.publicKey,
        adminUsdtTokenAccount: admin_usdt_token_account.address,
        protocolUsdtPoolPda: protocol_usdt_pool_pda,
        usdtMint: usdt_mint,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .rpc();

    const ico_info = await program.account.icoInfo.fetch(ico_info_pda);

    const ico_state = await program.account.icoState.fetch(ico_state_pda);
    const admin_usdt_token_balance_info = await provider.connection.getTokenAccountBalance(admin_usdt_token_account.address)
    assert.equal(admin_usdt_token_balance_info.value.amount.toString(), ico_state.totalUsdt.toString(), "USDT should be withdrawn to admin wallet");
  });
});
