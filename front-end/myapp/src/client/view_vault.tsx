let targetKeyString = "3AbLBHMDojuSUzj16tHRPovFs7AiX11fJFeJJFTqPkUE";
let targetKey = new anchor.web3.PublicKey(targetKeyString);
const [playerDataPda, bump1] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("vault")],
  pg.program.programId
);
const [targetDataPda, bump2] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("target","utf8"), pg.wallet.publicKey.toBuffer()],
  pg.program.programId
);
let txHash = await pg.program.methods
    .lockTarget(targetKey)
    .accounts({
      target: targetDataPda,
      signer: pg.wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();
console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
await pg.connection.confirmTransaction(txHash);
let gameDataAccount = await pg.program.account.userInfor.fetch(playerDataPda);
console.log(
  "The vault currently have " +
    gameDataAccount.assetAccount +
    " VND in the on chain account."
);