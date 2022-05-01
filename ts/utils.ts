import { Connection, Keypair, PublicKey } from "@solana/web3.js";
//@ts-expect-error missing types
import * as BufferLayout from "buffer-layout";

import * as fs from "fs";

export const logError = (msg: string) => {
  console.log(`\x1b[31m${msg}\x1b[0m`);
};

export const writePublicKey = (publicKey: PublicKey, name: string) => {
  fs.writeFileSync(
    `./keys/${name}_pub.json`,
    JSON.stringify(publicKey.toString())
  );
};

export const getPublicKey = (name: string) =>
  new PublicKey(
    JSON.parse(fs.readFileSync(`./keys/${name}_pub.json`) as unknown as string)
  );

export const getPrivateKey = (name: string) =>
  Uint8Array.from(
    JSON.parse(fs.readFileSync(`./keys/${name}.json`) as unknown as string)
  );

export const getKeypair = (name: string) =>
  new Keypair({
    publicKey: getPublicKey(name).toBytes(),
    secretKey: getPrivateKey(name),
  });

export const getProgramId = () => {
  try {
    return getPublicKey("program");
  } catch (e) {
    logError("Given programId is missing or incorrect");
    process.exit(1);
  }
};

export const getTerms = (): {
  aliceExpectedAmount: number;
  bobExpectedAmount: number;
} => {
  return JSON.parse(fs.readFileSync(`./terms.json`) as unknown as string);
};

export const getTokenBalance = async (
  pubkey: PublicKey,
  connection: Connection
) => {
  return parseInt(
    (await connection.getTokenAccountBalance(pubkey)).value.amount
  );
};

/**
 * Layout for a public key
 */
const publicKey = (property = "publicKey") => {
  return BufferLayout.blob(32, property);
};

/**
 * Layout for a 64bit unsigned value
 */
const uint64 = (property = "uint64") => {
  return BufferLayout.blob(8, property);
};

export const ESCROW_ACCOUNT_DATA_LAYOUT = BufferLayout.struct([
  BufferLayout.u8("isInitialized"),
  BufferLayout.u8("sol_dir"),
  uint64("sol_amount"),
  BufferLayout.u8("amount_x"),
  BufferLayout.u8("amount_y"),
  publicKey("initPubkey"),
  publicKey("takerPubkey"),
  publicKey("initializerPubkey1"),
  publicKey("initializerPubkey2"),
  publicKey("initializerPubkey3"),
  publicKey("initializerPubkey4"),
  publicKey("initializerPubkey5"),
  publicKey("initializerPubkey6"),
  publicKey("initializerPubkey7"),
  publicKey("initializerPubkey8"),
  publicKey("initializerPubkey9"),
  publicKey("initializerPubkey10"),
  publicKey("initializerPubkey11"),
  publicKey("initializerPubkey12"),
  publicKey("initializerPubkey13"),
  publicKey("initializerPubkey14"),
  publicKey("initializerPubkey15"),
  publicKey("initializerPubkey16"),
  publicKey("initializerPubkey17"),
  publicKey("initializerPubkey18"),
  publicKey("takerPubkey1"),
  publicKey("takerPubkey2"),
  publicKey("takerPubkey3"),
  publicKey("takerPubkey4"),
  publicKey("takerPubkey5"),
  publicKey("takerPubkey6"),
  publicKey("takerPubkey7"),
  publicKey("takerPubkey8"),
  publicKey("takerPubkey9"),
  publicKey("takerPubkey10"),
  publicKey("takerPubkey11"),
  publicKey("takerPubkey12"),
  publicKey("takerPubkey13"),
  publicKey("takerPubkey14"),
  publicKey("takerPubkey15"),
  publicKey("takerPubkey16"),
  publicKey("takerPubkey17"),
  publicKey("takerPubkey18"),
  publicKey("tempPubkey1"),
  publicKey("tempPubkey2"),
  publicKey("tempPubkey3"),
  publicKey("tempPubkey4"),
  publicKey("tempPubkey5"),
  publicKey("tempPubkey6"),
  publicKey("tempPubkey7"),
  publicKey("tempPubkey8"),
  publicKey("tempPubkey9"),
  uint64("lamports_1"),
  uint64("lamports_2"),
  uint64("lamports_3"),
  uint64("lamports_4"),
  uint64("lamports_5"),
  uint64("lamports_6"),
  uint64("lamports_7"),
  uint64("lamports_8"),
  uint64("lamports_9"),
  uint64("lamports_10"),
  uint64("lamports_11"),
  uint64("lamports_12"),
  uint64("lamports_13"),
  uint64("lamports_14"),
  uint64("lamports_15"),
  uint64("lamports_16"),
  uint64("lamports_17"),
  uint64("lamports_18")
]);

export interface EscrowLayout {
  isInitialized: number;
  initializerPubkey: Uint8Array;
  initializerReceivingTokenAccountPubkey: Uint8Array;
  initializerTempTokenAccountPubkey: Uint8Array;
  expectedAmount: Uint8Array;
}