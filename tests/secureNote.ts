import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SecureNote } from "../target/types/secure_note";

describe("secureNote", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SecureNote as Program<SecureNote>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
