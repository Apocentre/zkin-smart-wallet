import * as anchor from "@coral-xyz/anchor";
import {buildBn128, utils} from "ffjavascript";
import {createWallet} from "./common.js";
import {g1Uncompressed, g2Uncompressed, to32ByteBuffer} from "./utils/zk.js";
import {convert_proof} from "zkin-crypto-wasm";
import proof from "./proof.json" assert {type: "json"}
import publicSignals from "./public.json" assert {type: "json"}
import {expect} from "./utils/solana-chai.js";


const {unstringifyBigInts} = utils;

describe("Create wallet", async () => {  
  it("should create a new wallet", async () => {  
    const curve = await buildBn128();
    const proofProc = unstringifyBigInts(proof);

    // Tranform data to the correct shape and format
    let proofA = g1Uncompressed(curve, proofProc.pi_a);
    proofA = convert_proof(proofA);
    const proofB = g2Uncompressed(curve, proofProc.pi_b);
    const proofC = g1Uncompressed(curve, proofProc.pi_c);

    // replace the big int values of address and modulo with the [u8; 32] which is the hex encoded value in bytes
    publicSignals.splice(
      244,
      153,
      ...to32ByteBuffer(publicSignals[publicSignals.length - 2]),
      ...to32ByteBuffer(publicSignals[publicSignals.length - 1]),
    )

    await createWallet(proofA, proofB, proofC, publicSignals);
  })
});
