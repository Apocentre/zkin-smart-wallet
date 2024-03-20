import * as anchor from "@coral-xyz/anchor";
import Web3Pkg from "@apocentre/solana-web3";
import {createAndSendV0Tx} from "./utils/tx.js";
import * as accounts from "./helpers/accounts.js";

const Web3 = Web3Pkg.default;
const {SystemProgram} = anchor.web3

export const createWallet = async (proofA, proofB, proofC, pubInputs) => {
  const walletAddress = pubInputs.slice(244, 276);
  const provider = anchor.AnchorProvider.local();
  const program = anchor.workspace.ZkinSmartwallet;
  const owner = provider.wallet.payer;
  const web3 = Web3(owner.publicKey)
  const wallet = accounts.wallet(walletAddress, program.programId)[0];

  const ix = await program.methods
  .createWallet(walletAddress, proofA, proofB, proofC, pubInputs)
  .accounts({
    wallet,
    owner: owner.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .instruction();

  const cbIx = web3.getComputationBudgetIx(1_000_000);

  await createAndSendV0Tx(
    provider,
    [cbIx, ix],
    owner.publicKey,
    [owner]
  );
}
