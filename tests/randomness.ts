import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Randomness } from "../target/types/randomness";
import { getHousePDA } from "./utils";
import {
  ASSOCIATED_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
  AttestationQueueAccount,
  FunctionAccount,
  FunctionRequestAccount,
  NativeMint,
  SwitchboardProgram,
} from "@switchboard-xyz/solana.js";
import { parseRawMrEnclave } from "@switchboard-xyz/common";

const MRENCLAVE = parseRawMrEnclave(
  "0x0162074de74faf6e896b6c0b60341e0edc5470adee26fce7297ccbed306537db",
  true
);

describe("randomness", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.Randomness as Program<Randomness>;
  const wallet = provider.wallet;

  const housePDA = getHousePDA(program.programId);
  const houseTokenWallet = anchor.utils.token.associatedAddress({
    mint: NativeMint.address,
    owner: housePDA,
  });

  let switchboardProgram;
  let switchboard;
  let functionAccount;
  let functionWallet;

  before(async () => {
    switchboardProgram = await SwitchboardProgram.fromProvider(
      program.provider as anchor.AnchorProvider
    );

    switchboard = await AttestationQueueAccount.bootstrapNewQueue(
      switchboardProgram
    );

    // [functionAccount] =
    //   await switchboard.attestationQueue.account.createFunction({
    //     name: "test function",
    //     metadata: "this function handles XYZ for my protocol",
    //     schedule: "15 * * * * *",
    //     container: "switchboardlabs/basic-oracle-function",
    //     version: "latest",
    //     mrEnclave: MRENCLAVE,
    //   });

    // functionWallet = await functionAccount.wallet;
  });

  it("House init!", async () => {
    const tx = await program.methods
      .initHouse(100)
      .accounts({
        authority: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        house: housePDA,
        function: functionAccount.publicKey,
        escrowWallet: functionWallet.publicKey,
        tokenWallet: functionWallet.tokenWallet,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        mint: NativeMint.address,
        houseTokenWallet: houseTokenWallet,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
