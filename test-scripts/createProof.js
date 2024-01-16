const snarkjs = require("snarkjs");
const {readFile} = require("fs/promises");
const path = require("path");
const {buildBn128, utils} = require("ffjavascript");

const {unstringifyBigInts} = utils;

const wasmPath = path.join(__dirname, "../../zkin-circuit/build/zkin_js/", "zkin.wasm");
const zkeyPath = path.join(__dirname, "../../zkin-circuit/build/", "ZkIn_0001.zkey");
const inputPath = path.join(__dirname, "../../zkin-circuit/inputs/", "zkin.json");

const main = async () => {
  const input = JSON.parse(await readFile(inputPath));
  let {proof, publicSignals} = await snarkjs.groth16.fullProve(input, wasmPath, zkeyPath);
  console.log(">>>>", publicSignals)
}

main()
.then(() => console.log("Success"))
.catch((error) => console.log("Error: ", error))
