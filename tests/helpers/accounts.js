import * as anchor from "@coral-xyz/anchor";

const {PublicKey, Keypair} = anchor.web3;
const utf8 = anchor.utils.bytes.utf8;

export const state = () => Keypair.generate()

export const zkp = (walletAddress, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("zkp"), Buffer.from(walletAddress)],
  programId
)

export const wallet = (walletAddress, programId) => PublicKey.findProgramAddressSync(
  [Buffer.from(walletAddress)],
  programId
)
