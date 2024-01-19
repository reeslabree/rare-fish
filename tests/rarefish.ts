import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Rarefish } from "../target/types/rarefish";

describe("rarefish", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Rarefish as Program<Rarefish>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("This is a test test!", async () => {
    const tx = await program.methods.initialize().rpc();

    console.log("tx signature: ", tx);

    tx !== null ? console.log("success!") : console.log("failure!");
  });
});
