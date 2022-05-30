import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Test001SmartContract } from "../target/types/test001_smart_contract";

describe("test001_smart_contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Test001SmartContract as Program<Test001SmartContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
