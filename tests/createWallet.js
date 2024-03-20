import * as anchor from "@coral-xyz/anchor";
import {createWallet} from "./common.js"
import {buildBn128, utils} from "ffjavascript";
import {expect} from "./utils/solana-chai.js";

const {unstringifyBigInts} = utils;

const proof = {"pi_a":["20149639667624205476492861624756890558020441784676768981081968656233648716720","1299131889581230457619023461055794439291975684592303291944380312182172292773","1"],"pi_b":[["1589550127755022171007822207147022972537909301598495356951923591441437807810","5963444875394524661910956825063191667270591728331629820207046776152445001511"],["11157748767207118776740815382608319370129403572108911684965555536852594479378","8087440429666220320311296264098613101598073195341940010168180814047224018235"],["1","0"]],"pi_c":["9042951160887740851424770862968850382714951844154129274885760592843029517709","4669774116540053424957252962626009305907415388199170873253028844450385086122","1"],"protocol":"groth16"};
let publicSignals = ["104","116","116","112","115","58","47","47","97","99","99","111","117","110","116","115","46","103","111","111","103","108","101","46","99","111","109","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","0","52","56","51","50","49","50","53","52","56","48","55","56","45","113","99","104","49","103","115","118","115","108","115","114","117","113","51","50","49","104","50","108","51","98","104","101","97","112","57","110","97","49","111","103","51","46","97","112","112","115","46","103","111","111","103","108","101","117","115","101","114","99","111","110","116","101","110","116","46","99","111","109","0","0","0","0","0","0","57","102","97","49","56","54","54","102","55","99","52","102","50","55","99","102","48","54","97","54","53","97","102","102","56","48","50","98","101","54","55","48","102","55","51","54","102","52","55","56","55","55","53","98","50","50","48","49","100","102","98","98","102","56","55","52","53","57","99","98","56","102","98","57","0","0","0","0","0","0","0","0","0","0","0","0","0","0","49","55","48","50","50","57","51","54","55","50","14677101215157395175290708317047060803293912391017976749475749154361890907024","7579428519519079369887341705711273851487020884588892150709401894843040258070"];

const g1Uncompressed = (curve, p1Raw) => {
  const p1 = curve.G1.fromObject(p1Raw);
  const buff = new Uint8Array(64); // 64 bytes for G1 uncompressed
  curve.G1.toRprUncompressed(buff, 0, p1);

  return Buffer.from(buff);
}

const g2Uncompressed = (curve, p2Raw) => {
  const p2 = curve.G2.fromObject(p2Raw);
  const buff = new Uint8Array(128); // 128 bytes for G2 uncompressed
  curve.G2.toRprUncompressed(buff, 0, p2);

  return Buffer.from(buff);
}

const to32ByteBuffer = (num) => {
  const result = new Uint8Array(32);
  let i = 0;

  while (num > 0n) {
    result[i] = Number(num % 256n);
    num = num / 256n;
    i += 1;
  }

  return result;
}

describe("Create wallet", () => {  
  it("should create a new wallet", async () => {  
    const curve = await buildBn128();
    const proofProc = unstringifyBigInts(proof);
    publicSignals = unstringifyBigInts(publicSignals)

    // Tranform data to the correct shape and format
    const proofA = Array.from(g1Uncompressed(curve, proofProc.pi_a));
    const proofB = Array.from(g2Uncompressed(curve, proofProc.pi_b));
    const proofC = Array.from(g1Uncompressed(curve, proofProc.pi_c));
    const walletAddress = "C32Ad3bkok1cJ";

    await createWallet(walletAddress, proofA, proofB, proofC, publicSignals);
  })
});

