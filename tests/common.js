import * as anchor from "@coral-xyz/anchor";
import {createAndSendV0Tx} from "./utils/tx.js";
import * as accounts from "./helpers/accounts.js";
const {SystemProgram} = anchor.web3

export const createWallet = async (walletAddress, proofA, proofB, proofC, pubInputs) => {
  const provider = anchor.AnchorProvider.local();
  const program = anchor.workspace.ZkinSmartwallet;
  const owner = provider.wallet.payer;
  const wallet = accounts.wallet(walletAddress, program.programId)[0];

  // console.log(">>>>>>>>", walletAddress, proofA, proofB, proofC, pubInputs)

  const ix = await program.methods
  .createWallet(walletAddress, proofA, proofB, proofC, pubInputs)
  .accounts({
    wallet,
    owner: owner.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .instruction();

  await createAndSendV0Tx(
    provider,
    [ix],
    owner.publicKey,
    [owner]
  );
}
