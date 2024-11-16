import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { StakingBasedConnectionSystem } from "../target/types/staking_based_connection_system";
import assert from "assert";

describe("staking_based_connection_system", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .StakingBasedConnectionSystem as Program<StakingBasedConnectionSystem>;

  it("should create a user profile successfully", async () => {
    // Generate a new keypair for the user
    // const userKeypair = anchor.web3.Keypair.generate();
    const [userPda, _] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    console.log(userPda);
    console.log();
    
    const name = "Alice";
    // const dob = "2000-01-01";
    // const gender = "Female";

    // Fund the user's wallet
    // const airdropSignature = await provider.connection.requestAirdrop(
    //   userKeypair.publicKey,
    //   anchor.web3.LAMPORTS_PER_SOL
    // );
    // await provider.connection.confirmTransaction(airdropSignature);

    // Call the create_profile instruction
    await program.methods
      .createProfile(name)
      .rpc();
      // .accounts({
      //   user: userPda,
      //   signer: provider.wallet.publicKey,
      //   systemProgram: SystemProgram.programId,
      // })
      // .signers([userKeypair])

    // Fetch the created user account
    const userAccount = await program.account.user.all();

    // // Assertions
    // assert.equal(userAccount.name, name);
    // // assert.equal(userAccount.dob, dob);
    // // assert.equal(userAccount.gender, gender);
    // assert.equal(userAccount.reqCount, 0);
    // assert.equal(userAccount.reqSent, 0);
    // assert.strictEqual(
    //   userAccount.userPubkey.toBase58(),
    //   userKeypair.publicKey.toBase58()
    // );

    console.log("User profile created successfully:", userAccount);
  });
});
