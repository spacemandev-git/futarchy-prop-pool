import { Connection, PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { PROPOSER_KEY, MTN_MINT } from "./consts";


async function main() {
    const connection = new Connection("http://localhost:8899", "confirmed");
    console.log(await connection.getBalance(PROPOSER_KEY.publicKey));
    const tokenMint = new PublicKey(MTN_MINT);
    const tokenAccount = await getOrCreateAssociatedTokenAccount(connection, PROPOSER_KEY, tokenMint, PROPOSER_KEY.publicKey);
    console.log(tokenAccount.amount.toString());
}

main();