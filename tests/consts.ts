import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { Keypair } from "@solana/web3.js";

export const MTN_MINT = "mtnc7NNSpAJuvYNmayXU63WhWZGgFzwQ2yeYWqemeta";
export const USDC_MINT = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

export const PROPOSER_KEY = Keypair.fromSecretKey(bs58.decode(process.env.PROPOSER_KEY!));
export const MTN_WALLET = Keypair.fromSecretKey(bs58.decode(process.env.MTN_WALLET_KEY!));
export const USDC_WALLET = Keypair.fromSecretKey(bs58.decode(process.env.USDC_WALLET_KEY!));



