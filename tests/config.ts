import {
    workspace,
    AnchorProvider,
    Program,
} from "@coral-xyz/anchor";

import { CreIcoSolana } from "../target/types/cre_ico_solana";

// Configure the client to use the local cluster.
export const provider = AnchorProvider.env();

export const program = workspace.CreIcoSolana as Program<CreIcoSolana>;
