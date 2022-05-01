import {
    Connection,
    LAMPORTS_PER_SOL,
    PublicKey,
    Signer,
  } from "@solana/web3.js";
  
  import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
  import {
    getKeypair,
    getPublicKey,
    getTokenBalance,
    writePublicKey,
  } from "./utils";
  
  const createMint = (
    connection: Connection,
    { publicKey, secretKey }: Signer
  ) => {
    return Token.createMint(
      connection,
      {
        publicKey,
        secretKey,
      },
      publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );
  };
  
  const setupMint = async (
    name: string,
    connection: Connection,
    alicePublicKey: PublicKey,
    bobPublicKey: PublicKey,
    escrowPublicKey: PublicKey,
    clientKeypair: Signer
  ): Promise<[Token, PublicKey, PublicKey, PublicKey]> => {
    console.log(`Creating Mint ${name}...`);
    const mint = await createMint(connection, clientKeypair);
    writePublicKey(mint.publicKey, `mint_${name.toLowerCase()}`);
  
    console.log(`Creating Alice TokenAccount for ${name}...`);
    const aliceTokenAccount = await mint.createAccount(alicePublicKey);
    writePublicKey(aliceTokenAccount, `alice_${name.toLowerCase()}`);
  
    console.log(`Creating Bob TokenAccount for ${name}...`);
    const bobTokenAccount = await mint.createAccount(bobPublicKey);
    writePublicKey(bobTokenAccount, `bob_${name.toLowerCase()}`);
  
    console.log(`Creating Escrow TokenAccount for ${name}...`);
    const escrowTokenAccount = await mint.createAccount(escrowPublicKey);
    writePublicKey(escrowTokenAccount, `escrow_${name.toLowerCase()}`);

    return [mint, aliceTokenAccount, bobTokenAccount, escrowTokenAccount];
  };
  
  const setup = async () => {
    const alicePublicKey = getPublicKey("alice");
    const bobPublicKey = getPublicKey("bob");
    const escrowPublicKey = getPublicKey("escrow");
    const clientKeypair = getKeypair("id");
  
    // const connection = new Connection("http://localhost:8899", "confirmed");
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");
    // const connection = new Connection("https://api.testnet.solana.com", "confirmed");
    
    // console.log("Requesting SOL for Alice...");
    // some networks like the local network provide an airdrop function (mainnet of course does not)

    // await connection.requestAirdrop(alicePublicKey, LAMPORTS_PER_SOL * 100);
    // console.log("Requesting SOL for Bob...");
    // await connection.requestAirdrop(bobPublicKey, LAMPORTS_PER_SOL * 100);
    // console.log("Requesting SOL for Client...");
    // await connection.requestAirdrop(
    //   clientKeypair.publicKey,
    //   LAMPORTS_PER_SOL * 100
    // );
  //=============== alice mint X ================================
    const [mintX1, aliceTokenAccountForX1, bobTokenAccountForX1, escrowTokenAccountForX1] = await setupMint(
      "X1",
      connection,
      alicePublicKey,
      bobPublicKey,
      escrowPublicKey,
      clientKeypair
    );
    console.log("Sending X1 to Alice's X1 TokenAccount... 1");
    await mintX1.mintTo(aliceTokenAccountForX1, clientKeypair.publicKey, [], 1);

    const [mintX2, aliceTokenAccountForX2, bobTokenAccountForX2, escrowTokenAccountForX2] = await setupMint(
      "X2",
      connection,
      alicePublicKey,
      bobPublicKey,
      escrowPublicKey,
      clientKeypair
    );
    console.log("Sending X2 to Alice's X2 TokenAccount... 1");
    await mintX2.mintTo(aliceTokenAccountForX2, clientKeypair.publicKey, [], 1);

    const [mintX3, aliceTokenAccountForX3, bobTokenAccountForX3, escrowTokenAccountForX3] = await setupMint(
      "X3",
      connection,
      alicePublicKey,
      bobPublicKey,
      escrowPublicKey,
      clientKeypair
    );
    console.log("Sending X3 to Alice's X3 TokenAccount... 1");
    await mintX3.mintTo(aliceTokenAccountForX3, clientKeypair.publicKey, [], 1);

  //============= Bob mint Y =====================================
    const [mintY1, aliceTokenAccountForY1, bobTokenAccountForY1, escrowTokenAccountForY1] = await setupMint(
      "Y1",
      connection,
      alicePublicKey,
      bobPublicKey,
      escrowPublicKey,
      clientKeypair
    );
    console.log("Sending Y1 to Bob's Y1 TokenAccount...");
    await mintY1.mintTo(bobTokenAccountForY1, clientKeypair.publicKey, [], 1);

    const [mintY2, aliceTokenAccountForY2, bobTokenAccountForY2, escrowTokenAccountForY2] = await setupMint(
      "Y2",
      connection,
      alicePublicKey,
      bobPublicKey,
      escrowPublicKey,
      clientKeypair
    );
    console.log("Sending Y2 to Bob's Y2 TokenAccount...");
    await mintY2.mintTo(bobTokenAccountForY2, clientKeypair.publicKey, [], 1);
 //================ Print ========================================= 
    console.log("✨Setup complete✨\n");
    
    console.table([
      {
        "Alice Token Account X": await getTokenBalance(
          aliceTokenAccountForX1,
          connection
        ),
        "Alice Token Account Y": "",
        "Bob Token Account X": "",
        "Bob Token Account Y": "",
      },
      {
        "Alice Token Account X": await getTokenBalance(
          aliceTokenAccountForX2,
          connection
        ),
        "Alice Token Account Y": "",
        "Bob Token Account X": "",
        "Bob Token Account Y": "",
      },
      {
        "Alice Token Account X": await getTokenBalance(
          aliceTokenAccountForX3,
          connection
        ),
        "Alice Token Account Y": "",
        "Bob Token Account X": "",
        "Bob Token Account Y": "",
      },
      {
        "Alice Token Account X": "",
        "Alice Token Account Y": "",
        "Bob Token Account X": "",
        "Bob Token Account Y": await getTokenBalance(
          bobTokenAccountForY1,
          connection
        ),
      },
      {
        "Alice Token Account X": "",
        "Alice Token Account Y": "",
        "Bob Token Account X": "",
        "Bob Token Account Y": await getTokenBalance(
          bobTokenAccountForY2,
          connection
        ),
      },
    ]);
    console.log("");
  };
  
  setup();
