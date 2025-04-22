import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { LuzidSdk, Cluster, AccountModification } from "@luzid/sdk";
import { Keypair, PublicKey, Connection } from "@solana/web3.js";
import { getAssociatedTokenAddress } from "@solana/spl-token";


main().then(() => {
    console.log("Test completed");
}).catch((error) => {
    console.error("Test failed:", error);
});

async function main() {
    try {
        console.log("Starting Luzid test...");
        const keypair = Keypair.fromSecretKey(bs58.decode(process.env.PROPOSER_KEY!));
        const luzid = new LuzidSdk();
        
        const validatorInfo = await luzid.validator.getValidatorInfo();
        console.log("Validator info:", validatorInfo);

        // BONK token mint address on mainnet
        const BONK_MINT = new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263");
        
        // Get the associated token account for BONK
        const associatedTokenAddress = await getAssociatedTokenAddress(BONK_MINT, keypair.publicKey);
        console.log(`Associated token address for BONK: ${associatedTokenAddress.toString()}`);
        
        // Clone the token account from mainnet
        console.log(`Cloning BONK token account from mainnet for ${keypair.publicKey.toString()}`);
        
        // Check if the account exists on mainnet before cloning
        const connection = new Connection(process.env.MAINNET_RPC_URL!);
        const accountInfo = await connection.getAccountInfo(associatedTokenAddress);
        
        if (accountInfo) {
            await luzid.mutator.cloneAccount(
                Cluster.MainnetBeta, 
                associatedTokenAddress.toString()
            );
            console.log("BONK token account cloned successfully");


            // Get account data after cloning
            const accountInfoResponse = await luzid.rpc.getAccountInfo(
                Cluster.Development,
                associatedTokenAddress.toString(),
            );
            
            if (accountInfoResponse) {
                console.log("Successfully retrieved account info");
                // Extract data and parse the balance
                const data = Buffer.from(accountInfoResponse.account.data);
                
                // Amount is stored at position 64-72 (8 bytes for u64) in token accounts
                const amountBuffer = data.subarray(64, 72);
                const originalAmount = amountBuffer.readBigUInt64LE(0);
                console.log(`Original BONK balance: ${originalAmount.toString()}`);
                
                // Modify the token account to 10x the balance
                const newAmount = originalAmount * BigInt(10);
                console.log(`Setting new balance to: ${newAmount.toString()}`);
                
                // Create a buffer for the new amount
                const newAmountBuffer = Buffer.alloc(8);
                newAmountBuffer.writeBigUInt64LE(newAmount, 0);
                
                // Update the account data with the new amount
                const newData = Buffer.from(data);
                newAmountBuffer.copy(newData, 64);
                
                try {
                    await luzid.mutator.modifyAccount(
                        AccountModification.forAddr(associatedTokenAddress.toString())
                        .setData(Uint8Array.from(newData))
                    );  
                    console.log("Account balance multiplied by 10x");
                } catch (error) {
                    console.error("Error updating account:", error);
                    return;
                }
                
                // Verify the new balance
                const updatedAccountInfo = await luzid.rpc.getAccountInfo(
                    Cluster.Development,
                    associatedTokenAddress.toString()
                );
                
                if (updatedAccountInfo) {
                    const updatedData = Buffer.from(updatedAccountInfo.account.data);
                    const updatedAmountBuffer = updatedData.subarray(64, 72);
                    const updatedAmount = updatedAmountBuffer.readBigUInt64LE(0);
                    
                    console.log(`New BONK balance: ${updatedAmount.toString()}`);
                    console.log(`Verification: Original × 10 = ${originalAmount * BigInt(10)}`);
                    
                    if (updatedAmount === originalAmount * BigInt(10)) {
                        console.log("SUCCESS: Balance was successfully multiplied by 10!");
                    } else {
                        console.log("FAILED: Balance modification didn't work as expected");
                    }
                } else {
                    console.log("Could not verify the updated token account");
                }
            } else {
                console.log("Could not find the cloned token account");
            }
        } else {
            console.log("Token account does not exist on mainnet. Cannot clone.");
        }
    } catch (error) {
        console.error("Error in luzid test:", error);
    }
}