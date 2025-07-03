import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import wallet from "/home/anish/.config/solana/id.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("Fa83fsxVfa7WQ9vAZQB2Z2QdjJpJcUTWZsPPQg78aeLA");

// Recipient address
const to = new PublicKey("4Bt7w7N4ZC796UkhYhzbx9Z6x8mntXeco2tJCnwCDJd1"); //main phantom pubkey

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromWallet = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);
        console.error(`ata of from wallet: ${fromWallet.address}`);

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toWallet = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to);
        console.error(`ata of to wallet: ${toWallet.address}`);

        // Transfer the new token to the "toTokenAccount" we just created
        const txn = await transfer(connection, keypair, fromWallet.address, toWallet.address, keypair.publicKey, 1);
        console.error(`txn: ${txn}`);

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();