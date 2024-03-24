import * as anchor from "@coral-xyz/anchor";
import Web3Pkg from "@apocentre/solana-web3";
import {createAndSendV0Tx} from "./utils/tx.js";
import * as accounts from "./helpers/accounts.js";

const Web3 = Web3Pkg.default;
const {SystemProgram} = anchor.web3
const {BN} = anchor.default;
const BATCH_SIZE = 50;

export const createWallet = async (proofA, proofB, proofC, pubInputs) => {
  const walletAddress = pubInputs.slice(244, 276);
  const provider = anchor.AnchorProvider.local();
  const program = anchor.workspace.ZkinSmartwallet;
  const owner = provider.wallet.payer;
  const web3 = Web3(owner.publicKey)
  const wallet = accounts.wallet(walletAddress, program.programId)[0];
  const zkp = accounts.zkp(walletAddress, program.programId)[0];

  // we use 5 instruction to run the first transaction and 2 instructions to run the second
  const initZkpIx = await program.methods
  .initZkp(walletAddress, proofA, proofB, proofC, pubInputs, new BN(BATCH_SIZE))
  .accounts({
    zkp,
    owner: owner.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .instruction();

  let prepareZkpIxs = [];

  for (let i = 0; i < 3; i++) {
    const ix = await program.methods
    .prepareZkp(walletAddress)
    .accounts({zkp})
    .instruction();

    prepareZkpIxs.push(ix);
  }


  let cbIx = web3.getComputationBudgetIx(1_000_000);
  await createAndSendV0Tx(
    provider,
    [cbIx, initZkpIx, ...prepareZkpIxs],
    owner.publicKey,
    [owner]
  );

  prepareZkpIxs = [];
  for (let i = 0; i < 2; i++) {
    const ix = await program.methods
    .prepareZkp(walletAddress)
    .accounts({zkp})
    .instruction();

    prepareZkpIxs.push(ix);
  }

  const createWalletIx = await program.methods
  .createWallet(walletAddress)
  .accounts({
    wallet,
    zkp,
    owner: owner.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .instruction();

  cbIx = web3.getComputationBudgetIx(1_000_000);
  await createAndSendV0Tx(
    provider,
    [cbIx, ...prepareZkpIxs, createWalletIx],
    owner.publicKey,
    [owner]
  );
}
