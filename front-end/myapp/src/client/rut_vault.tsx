let targetKeyString = "3AbLBHMDojuSUzj16tHRPovFs7AiX11fJFeJJFTqPkUE";
let targetKey = new anchor.web3.PublicKey(targetKeyString);
const [vaultDataPda, bump1] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("vault")],
  pg.program.programId
);
const [targetDataPda, bump2] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("target","utf8"), pg.wallet.publicKey.toBuffer()],
  pg.program.programId
);
const [donatorDataPda, bump3] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("client","utf8"), pg.wallet.publicKey.toBuffer()],
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
  let gameDataAccount = await pg.program.account.userInfor.fetch(donatorDataPda);
  console.log(
    "Before summon you have " +
      gameDataAccount.assetAccount +
      " VND in your account."
  );
  let amount: number = 25000;
  const amountBN = new BN(amount);
  let txHash = await pg.program.methods
    .summonAsset(amountBN)
    .accounts({
      target: targetDataPda,
      donator: donatorDataPda,
      vault: vaultDataPda,
      signer: pg.wallet.publicKey,
    })
    .rpc();
  console.log(`Summon ${amount} VND from the vault`);
  console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  await pg.connection.confirmTransaction(txHash);
  gameDataAccount = await pg.program.account.userInfor.fetch(donatorDataPda);
  console.log(
    "After summon you have " +
      gameDataAccount.assetAccount +
      " VND in your account."
  );
} catch (e) {
  console.log(`${e}`);
}