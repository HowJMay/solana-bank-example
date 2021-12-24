import {
  AccountLayout,
  Token,
  TOKEN_PROGRAM_ID,
  NATIVE_MINT,
} from "@solana/spl-token";
import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
  SYSVAR_RENT_PUBKEY,
  Signer,
} from "@solana/web3.js";
import fs from "mz/fs";
import path from "path";
import BN = require("bn.js");
import * as borsh from "borsh";

import { get_payer, get_rpc_url, create_keypair_from_file } from "./utils";

/**
 * Connection to the network
 */
let connection: Connection;

/**
 * Keypair associated to the fees' payer
 */
let payer: Keypair;

/**
 * Bank's program id
 */
let program_id: PublicKey;

/**
 * The public key of the account that receive money
 */
let money_received_pubkey: PublicKey;

/**
 * Path to Solana program files
 */
const SOLANA_PROGRAM_PATH = path.resolve(__dirname, "../../dist/program");

/**
 * Path to program shared object file which should be deployed on chain.
 * This file is created when running either:
 *   - `npm run build:program-c`
 *   - `npm run build:program-rust`
 */
const PROGRAM_SO_PATH = path.join(
  SOLANA_PROGRAM_PATH,
  "solana_bank_example.so"
);

/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/solana_bank_example.so`
 */
const PROGRAM_KEYPAIR_PATH = path.join(
  SOLANA_PROGRAM_PATH,
  "solana_bank_example-keypair.json"
);

/**
 * The state of a money_receiver_account managed by the bank program
 */
class MONEY_RECEIVER_ACCOUNT {
  note = "";
  constructor(fields: { note: string } | undefined = undefined) {
    if (fields) {
      this.note = fields.note;
    }
  }
}

/**
 * Borsh schema definition for money_receiver accounts
 */
const MONEY_RECEIVER_SCHEMA = new Map([
  [MONEY_RECEIVER_ACCOUNT, { kind: "struct", fields: [["note", "string"]] }],
]);

/**
 * The expected size of each money_receiver account.
 */
const MONEY_RECEIVER_ACCOUNT_SIZE = borsh.serialize(
  MONEY_RECEIVER_SCHEMA,
  new MONEY_RECEIVER_ACCOUNT()
).length;

/**
 * Establish a connection to the cluster
 */
export async function establish_connection(): Promise<void> {
  const rpcUrl = await get_rpc_url();
  connection = new Connection(rpcUrl, "confirmed");
  const version = await connection.getVersion();
  console.log("Connection to cluster established:", rpcUrl, version);
}

/**
 * Establish an account to pay for everything
 */
export async function establish_payer(): Promise<void> {
  let fees = 0;
  if (!payer) {
    const { feeCalculator } = await connection.getRecentBlockhash();

    // Calculate the cost to fund the money reciever account
    fees += await connection.getMinimumBalanceForRentExemption(
      MONEY_RECEIVER_ACCOUNT_SIZE
    );

    // Calculate the cost of sending transactions
    fees += feeCalculator.lamportsPerSignature * 100; // wag

    payer = await get_payer();
  }

  let lamports = await connection.getBalance(payer.publicKey);
  if (lamports < fees) {
    // If current balance is not enough to pay for fees, request an airdrop
    const sig = await connection.requestAirdrop(
      payer.publicKey,
      fees - lamports
    );
    await connection.confirmTransaction(sig);
    lamports = await connection.getBalance(payer.publicKey);
  }

  console.log(
    "Using account",
    payer.publicKey.toBase58(),
    "containing",
    lamports / LAMPORTS_PER_SOL,
    "SOL to pay for fees"
  );
}

/**
 * Check if the Solana Bank BPF program has been deployed
 */
export async function check_program(): Promise<void> {
  // Read program id from keypair file
  try {
    const program_keypair = await create_keypair_from_file(
      PROGRAM_KEYPAIR_PATH
    );
    program_id = program_keypair.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/solana-bank-example.so\``
    );
  }

  // Check if the program has been deployed
  const program_info = await connection.getAccountInfo(program_id);
  if (program_info === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        "Program needs to be deployed with `solana program deploy dist/program/solana-bank-example.so`"
      );
    } else {
      throw new Error("Program needs to be built and deployed");
    }
  } else if (!program_info.executable) {
    throw new Error(`Program is not executable`);
  }

  console.log(`Using program ${program_id.toBase58()}`);

  // Derive the address (public key) of a money reciever account from the program so that it's easy to find later.
  const MONEY_RECEIVED_SEED = "I want money";
  money_received_pubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    MONEY_RECEIVED_SEED,
    program_id
  );

  // Check if the money reciever account has already been created
  const money_receiver_account = await connection.getAccountInfo(
    money_received_pubkey
  );
  if (money_receiver_account === null) {
    console.log(
      "Creating account",
      money_received_pubkey.toBase58(),
      "to receive money and note"
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      MONEY_RECEIVER_ACCOUNT_SIZE
    );

    const transaction = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payer.publicKey,
        basePubkey: payer.publicKey,
        seed: MONEY_RECEIVED_SEED,
        newAccountPubkey: money_received_pubkey,
        lamports,
        space: MONEY_RECEIVER_ACCOUNT_SIZE,
        programId: program_id,
      })
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
}

/**
 * Deposit token
 */
export async function deposit_token(): Promise<void> {
  console.log("deposit money to", money_received_pubkey.toBase58());
  var deposit_amount = 10;

  // transfer money to the sub-account
  const owner = Keypair.generate();
  const deposited_account_pubkey = await Token.createWrappedNativeAccount(
    connection,
    TOKEN_PROGRAM_ID,
    payer.publicKey,
    payer,
    (await connection.getMinimumBalanceForRentExemption(AccountLayout.span)) +
      deposit_amount
  );

  // trigger othe operations
  var deposit_amount_big_number = new BN(deposit_amount).toArray("le", 8);
  var enc = new TextEncoder();
  var deposit_note = enc.encode("deposit note");
  const instruction = new TransactionInstruction({
    keys: [
      { pubkey: money_received_pubkey, isSigner: false, isWritable: true },
      { pubkey: deposited_account_pubkey, isSigner: false, isWritable: true },
      { pubkey: payer.publicKey, isSigner: true, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: program_id,
    data: Buffer.from(
      Uint8Array.of(0, ...deposit_amount_big_number, ...deposit_note)
    ),
  });

  var signature = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer]
  );

  console.log("SUCCESS===========");
}

/**
 * Report the received note
 */
export async function report_received_note(): Promise<void> {
  const accountInfo = await connection.getAccountInfo(money_received_pubkey);
  if (accountInfo === null) {
    throw "Error: cannot find the money received account";
  }
  const received_info = borsh.deserialize(
    MONEY_RECEIVER_SCHEMA,
    MONEY_RECEIVER_ACCOUNT,
    accountInfo.data
  );
  console.log(
    "Account: '%s'receives note:",
    money_received_pubkey.toBase58(),
    received_info.note
  );
}
