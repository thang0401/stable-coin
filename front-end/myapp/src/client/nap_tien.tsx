let targetKeyString = "6hMc8mdEk31DPAU9DnMefnuMkUJujwynasccbXzcJ1fn";
let targetKey = new anchor.web3.PublicKey(targetKeyString);
const [playerDataPda, bump1] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("client","utf8"), targetKey.toBuffer()],
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
let targetDataAccount = await pg.program.account.userTarget.fetch(targetDataPda);
console.log(
  "You targeting to Pubkey: " +
    targetDataAccount.assetTarget +
    " after your command."
);
try {
  let gameDataAccount = await pg.program.account.userInfor.fetch(playerDataPda);
  console.log(
    "Before deposit you have " +
      gameDataAccount.assetAccount +
      " VND in your account."
  );
  let amount: number = 100000;
  const amountBN = new BN(amount);
  let txHash = await pg.program.methods
    .depositAsset(amountBN)
    .accounts({
      target: targetDataPda,
      client: playerDataPda,
      signer: pg.wallet.publicKey,
    })
    .rpc();
  console.log(`Deposited ${amount} VND to your account`);
  console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  await pg.connection.confirmTransaction(txHash);
  gameDataAccount = await pg.program.account.userInfor.fetch(playerDataPda);
  console.log(
    "After deposit you have " +
      gameDataAccount.assetAccount +
      " VND in your account."
  );
} catch (e) {
  console.log(`${e}`);
}