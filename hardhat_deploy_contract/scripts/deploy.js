// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require("hardhat");
const {Buffer} = require('node:buffer');
const fs = require('fs');
const {TextEncoder} = require("util");
const { exit } = require("node:process");

async function main() {

  let txtFile = "bao_slice_2.txt";
  let proof = fs.readFileSync(txtFile);
  await hre.run("compile")
  const transactionResponse = await bao.save_proof(proof);
  const transactionReceipt = await transactionResponse.wait()
  console.log(transactionReceipt)
  console.log(transactionReceipt.events[0].args)
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
