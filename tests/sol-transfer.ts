import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolTransfer } from "../target/types/sol_transfer";

describe("sol-transfer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolTransfer as Program<SolTransfer>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    const dataAccountKP = anchor.web3.Keypair.generate();
    const user = anchor.web3.Keypair.fromSecretKey(new Uint8Array([168,169,72,53,157,208,199,2,172,124,71,118,95,24,70,61,19,132,19,86,225,71,181,186,231,173,6,37,169,99,155,205,147,69,201,253,198,229,153,216,64,160,193,156,0,147,228,162,189,226,35,26,128,30,108,246,153,209,113,215,215,112,75,235]));
    const tx = program.methods.myCommand(new anchor.BN(666))
      .accounts({
        dataAccount: dataAccountKP.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user, dataAccountKP])
      .rpc(); // for sending transaction.
    console.log("Your transaction signature", tx);
  });


  
  it("Test multiplicaion !", async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    const dataAccountKP = anchor.web3.Keypair.generate();
    const user = anchor.web3.Keypair.fromSecretKey(new Uint8Array([168,169,72,53,157,208,199,2,172,124,71,118,95,24,70,61,19,132,19,86,225,71,181,186,231,173,6,37,169,99,155,205,147,69,201,253,198,229,153,216,64,160,193,156,0,147,228,162,189,226,35,26,128,30,108,246,153,209,113,215,215,112,75,235]));
    const tx = program.methods.myMultiplication()
      .accounts({
        dataAccount1: new anchor.web3.PublicKey("2GP1714MWmjJRnv49DyQnjRKwfCVFANmetaEHaprJE1Y"),
        dataAccount2: new anchor.web3.PublicKey("DK5gm258PTN37JGjpob2ifGSwtqUULH2joXj81FddMTf"),
      })
      .signers([user])
      .rpc(); // for sending transaction.
    console.log("Your transaction signature", tx);
  });





  
});
