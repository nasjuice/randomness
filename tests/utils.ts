import * as anchor from "@coral-xyz/anchor";

export function getHousePDA(programID) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("house_")],
    programID
  )[0];
}
