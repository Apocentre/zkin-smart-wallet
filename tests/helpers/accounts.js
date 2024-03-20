import * as anchor from "@coral-xyz/anchor";

const {PublicKey, Keypair} = anchor.web3;
const utf8 = anchor.utils.bytes.utf8;

export const state = () => Keypair.generate()

export const wallet = (walletAddress, programId) => PublicKey.findProgramAddressSync(
  [Buffer.from(walletAddress)],
  programId
)
