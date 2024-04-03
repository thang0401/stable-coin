let targetKeyString = "HSaxGp1yjsBRcLLzxMQvdR3QXoVixtvY94TcU2wAyCNC";
let targetKey = new anchor.web3.PublicKey(targetKeyString);
const [playerDataPda, bump1] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("client","utf8"), targetKey.toBuffer()],
  pg.program.programId
);
const [targetDataPda, bump2] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("target","utf8"), pg.wallet.publicKey.toBuffer()],
  pg.program.programId
);
const [vaultDataPda, bump3] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("vault")],
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
let targetDataAccount = await pg.program.account.userTarget.fetch(targetDataPda);
console.log(
  "You targeting to Pubkey: " +
    targetDataAccount.assetTarget +
    " after your command."
);
try {
  let txHash = await pg.program.methods
    .clearUser()
    .accounts({
        target: targetDataPda,
        client: playerDataPda,
        signer: pg.wallet.publicKey,
      })
    .rpc();
  await pg.connection.confirmTransaction(txHash);
  console.log(`User of Pubkey: ${targetKey} has been deleted`);
  console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
} catch (e) {
  console.log(`${e}`);
}