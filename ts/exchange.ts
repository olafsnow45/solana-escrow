import { AccountLayout, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  LAMPORTS_PER_SOL,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import BN = require("bn.js");
import {
  EscrowLayout,
  ESCROW_ACCOUNT_DATA_LAYOUT,
  getKeypair,
  getProgramId,
  getPublicKey,
  getTerms,
  getTokenBalance,
  logError,
  writePublicKey,
} from "./utils";

const alice = async () => {
  // const terms = getTerms();
    
  const aliceKeypair = getKeypair("alice");
  const bobKeypair = getKeypair("bob");
  const escrowKeypair = getKeypair("escrow");
  const clientKeypair = getKeypair("id");

  // const connection = new Connection("https://api.testnet.solana.com", "confirmed");
  // const connection = new Connection("http://localhost:8899", "confirmed");
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");
  
  const escrowProgramId = getProgramId();

  const aliceXAmount = 3;
  const bobYAmount = 2;

  const aliceTokenAccountPubkey = [
    getPublicKey("alice_x1"),
    getPublicKey("alice_x2"),
    getPublicKey("alice_x3"),
    getPublicKey("alice_y1"),
    getPublicKey("alice_y2"),
    getPublicKey("alice_token1"),
    getPublicKey("alice_token2")
  ];
  const bobTokenAccountPubkey = [
    getPublicKey("bob_x1"),
    getPublicKey("bob_x2"),
    getPublicKey("bob_x3"),
    getPublicKey("bob_y1"),
    getPublicKey("bob_y2"),
    getPublicKey("bob_token1"),
    getPublicKey("bob_token2")
  ];
  const tempTokenAccountPubkey = [
    getPublicKey("escrow_x1"),
    getPublicKey("escrow_x2"),
    getPublicKey("escrow_x3"),
    getPublicKey("escrow_y1"),
    getPublicKey("escrow_y2"),
    getPublicKey("escrow_token1"),
    getPublicKey("escrow_token2")
  ];
  
  const PDA = await PublicKey.findProgramAddress(
    [Buffer.from("escrow")],
    escrowProgramId
  );

  const exchangeEscrowIx = new TransactionInstruction({
    programId: escrowProgramId,
    keys: [
      { pubkey: aliceKeypair.publicKey, isSigner: false, isWritable: true },
      { pubkey: bobKeypair.publicKey, isSigner: true, isWritable: true },
      { pubkey: escrowKeypair.publicKey, isSigner: false, isWritable: true },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: PDA[0], isSigner: false, isWritable: false },
      //--- a -> b --- NFT
      { pubkey: aliceTokenAccountPubkey[0], isSigner: false, isWritable: true },
      { pubkey: bobTokenAccountPubkey[0], isSigner: false, isWritable: true },
      { pubkey: tempTokenAccountPubkey[0], isSigner: false, isWritable: true },
      { pubkey: aliceTokenAccountPubkey[1], isSigner: false, isWritable: true },
      { pubkey: bobTokenAccountPubkey[1], isSigner: false, isWritable: true },
      { pubkey: tempTokenAccountPubkey[1], isSigner: false, isWritable: true },
      { pubkey: aliceTokenAccountPubkey[2], isSigner: false, isWritable: true },
      { pubkey: bobTokenAccountPubkey[2], isSigner: false, isWritable: true },
      { pubkey: tempTokenAccountPubkey[2], isSigner: false, isWritable: true },
      //--- a -> b --- FT
      { pubkey: aliceTokenAccountPubkey[5], isSigner: false, isWritable: true },
      { pubkey: bobTokenAccountPubkey[5], isSigner: false, isWritable: true },
      { pubkey: tempTokenAccountPubkey[5], isSigner: false, isWritable: true },

      //--- b -> a --- NFT
      { pubkey: aliceTokenAccountPubkey[3], isSigner: false, isWritable: true },
      { pubkey: bobTokenAccountPubkey[3], isSigner: false, isWritable: true },
      { pubkey: aliceTokenAccountPubkey[4], isSigner: false, isWritable: true },
      { pubkey: bobTokenAccountPubkey[4], isSigner: false, isWritable: true },
      //--- b -> a --- FT
      { pubkey: aliceTokenAccountPubkey[6], isSigner: false, isWritable: true },
      { pubkey: bobTokenAccountPubkey[6], isSigner: false, isWritable: true },

      { pubkey: SystemProgram.programId, isSigner:false, isWritable:false }
    ],
    data: Buffer.from(
      Uint8Array.of(
        1,  // escrow exchange

        1,  // sol_dir : alice -> bob
        ...new BN(2000000000).toArray("le", 8),     //sol_lamports

        ...new BN(4).toArray("le", 1),   //alice token amount
        ...new BN(1).toArray("le", 8),
        ...new BN(1).toArray("le", 8),
        ...new BN(1).toArray("le", 8),
        ...new BN(2000000000).toArray("le", 8),

        ...new BN(3).toArray("le", 1),     //bob token amount
        ...new BN(1).toArray("le", 8),
        ...new BN(1).toArray("le", 8),
        ...new BN(3000000000).toArray("le", 8),
      )
),
});

  const tx = new Transaction().add(
    exchangeEscrowIx
  );

  console.log("alicePubkey : ", aliceKeypair.publicKey.toBase58());
  console.log("bobPubkey : ", bobKeypair.publicKey.toBase58());

  console.log(tx);
  console.log("Sending Exchange's transaction...");
  await connection.sendTransaction(
    tx,
    [bobKeypair, escrowKeypair ],
    { skipPreflight: false, preflightCommitment: "confirmed" }
  );
  console.log("Exchange completed!");

};

alice();