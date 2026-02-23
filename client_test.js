const anchor = require("@coral-xyz/anchor");
const { TOKEN_PROGRAM_ID } = require("@solana/spl-token");

async function testEscrow() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    
    const program = anchor.workspace.SolanaEscrow;
    
    // Generate PDA for the vault
    const [vaultPDA, bump] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("escrow"), provider.wallet.publicKey.toBuffer()],
        program.programId
    );

    console.log("Vault PDA Address:", vaultPDA.toBase58());
}

testEscrow().catch(console.error);
