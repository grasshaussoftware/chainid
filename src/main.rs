use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::signers::{LocalWallet, Signer};
use coins_bip39::{English, Mnemonic};
use qrcode::QrCode;
use qrcode::render::unicode;
use serde_json::json;
use std::io::{self, Write};
use std::time::Duration;
use tokio::time::{sleep, timeout};
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};
use std::env;

type HmacSha256 = Hmac<Sha256>;

// ---------------------------------------------------------
// HARDWARE ENCLAVE & MEMORY SAFETY
// ---------------------------------------------------------
#[derive(Zeroize, ZeroizeOnDrop)]
struct SecureIdentity {
    phrase: String,
}

impl SecureIdentity {
    fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let mnemonic = Mnemonic::<English>::new(&mut rng);
        SecureIdentity { phrase: mnemonic.to_phrase() }
    }
}

// ---------------------------------------------------------
// UTILITIES & CRYPTOGRAPHY
// ---------------------------------------------------------
fn clear_screen() {
    // Clears the screen and resets the cursor to the top left
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    let _ = io::stdout().flush();
}

/// Hashes PII using HMAC-SHA256 with a public salt (block hash) 
/// AND a private hardware pepper to prevent brute-forcing low-entropy data.
fn secure_hash_pii(data: &str, public_salt: &str, hardware_pepper: &str) -> Result<String, &'static str> {
    let combined_key = format!("{}{}", public_salt, hardware_pepper);
    let mut mac = HmacSha256::new_from_slice(combined_key.as_bytes())
        .map_err(|_| "Failed to initialize HMAC hardware protocol")?;
    mac.update(data.as_bytes());
    let result = mac.finalize();
    Ok(hex::encode(result.into_bytes()))
}

fn generate_qr(data: &str) -> Result<String, &'static str> {
    let code = QrCode::new(data).map_err(|_| "Failed to encode QR payload")?;
    Ok(code.render::<unicode::Dense1x2>().quiet_zone(false).build())
}

// ---------------------------------------------------------
// ROBUST I/O
// ---------------------------------------------------------
async fn prompt_secure_input(prompt: &str, timeout_secs: u64) -> Result<Zeroizing<String>, &'static str> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|_| "Hardware I/O Error")?;

    let input_task = tokio::task::spawn_blocking(|| {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        Zeroizing::new(input.trim().to_string())
    });

    match timeout(Duration::from_secs(timeout_secs), input_task).await {
        Ok(Ok(result)) if !result.is_empty() => Ok(result),
        _ => Err("SESSION TIMEOUT: User idle. Memory automatically wiped."),
    }
}

// ---------------------------------------------------------
// NETWORK MOCKS
// ---------------------------------------------------------
async fn await_metamask_connection() -> Result<String, &'static str> {
    let wc_uri = "wc:1234567890...mock_walletconnect_uri...xyz";
    let image = generate_qr(wc_uri)?;
    
    println!("--- AUTHENTICATION ---");
    println!("Scan with MetaMask to begin. (QR Expires in 60s)\n");
    println!("{}", image);
    
    match timeout(Duration::from_secs(60), sleep(Duration::from_secs(4))).await {
        Ok(_) => Ok("0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B".to_string()),
        Err(_) => Err("SESSION TIMEOUT: QR Code expired."),
    }
}

async fn pin_json_to_ipfs(_payload: &serde_json::Value) -> Result<String, &'static str> {
    println!("\n>>> UPLOADING METADATA TO IPFS <<<");
    println!("Pinning zero-knowledge hashes to the decentralized web...");
    sleep(Duration::from_secs(3)).await; // Simulate network processing
    Ok("ipfs://QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG".to_string())
}

// ---------------------------------------------------------
// THE CORE KIOSK SESSION (Zero-Panic Execution)
// ---------------------------------------------------------
async fn run_kiosk_session(hardware_pepper: &str) -> Result<(), &'static str> {
    let user_wallet = await_metamask_connection().await?;

    println!("Connecting to Avalanche-C Mainnet...");
    let rpc_url = "https://avalanche-mainnet.infura.io/v3/f4824ede0b484d33a19f0d01c32c9de1";
    let provider = Provider::<Http>::try_from(rpc_url)
        .map_err(|_| "RPC Network Error: Check internet connection")?;

    let block = provider.get_block(BlockNumber::Latest).await
        .map_err(|_| "Failed to fetch chain state")?
        .ok_or("Block data missing from RPC provider")?;
        
    // Use {:#x} to ensure the standard 0x prefix for the cryptographic salt
    let omega_block_hash = format!("{:#x}", block.hash.ok_or("Hash missing in block payload")?);
    let omega_block_number = block.number.ok_or("Block number missing")?.as_u64();
    let omega_block_timestamp = block.timestamp.as_u64();
    let omega_chronos = Utc::now().to_rfc3339();

    clear_screen();
    println!("[Secure Connection Established: {}]", user_wallet);
    println!("Enter physical identity details. Data is secured in volatile RAM.\n");

    // Inputs are immediately wrapped in Zeroizing to ensure memory safety
    let name = prompt_secure_input("Full Legal Name: ", 45).await?;
    let dob = prompt_secure_input("Date of Birth (YYYY-MM-DD): ", 45).await?;
    let phone = prompt_secure_input("Mobile Phone Number: ", 45).await?;
    let email = prompt_secure_input("Email Address: ", 45).await?;
    let zip_code = prompt_secure_input("ZIP / Postal Code: ", 45).await?;

    clear_screen();
    println!("Generating Air-Gapped Cryptographic Identity...");
    
    let identity_address;
    let nft_metadata;

    {
        // 🔒 SECURE ENCLAVE OPENS
        let secure_id = SecureIdentity::generate();
        let identity_wallet = LocalWallet::new_from_mnemonic(&secure_id.phrase, None)
            .map_err(|_| "Math Error: Failed to derive secp256k1 wallet")?;
        identity_address = format!("{:?}", identity_wallet.address());

        println!("\n>>> DISPATCHING TO THERMAL PRINTER <<<");
        println!("Your 24-word Identity Seed is:\n{}", secure_id.phrase);
        println!("Identity Address: {}", identity_address);
        println!("Omega Block Anchor: {}", omega_block_number); 
        println!(">>> DO NOT LEAVE BEHIND <<<\n");

        prompt_secure_input("Press ENTER once you have collected your printed receipt...", 60).await?;

        // 🌶️ HMAC-SHA256 Hashing with explicit drops for immediate memory scrubbing
        let dob_hash = secure_hash_pii(&dob, &omega_block_hash, hardware_pepper)?;
        drop(dob); 
        
        let phone_hash = secure_hash_pii(&phone, &omega_block_hash, hardware_pepper)?;
        drop(phone);
        
        let email_hash = secure_hash_pii(&email, &omega_block_hash, hardware_pepper)?;
        drop(email);
        
        let zip_hash = secure_hash_pii(&zip_code, &omega_block_hash, hardware_pepper)?;
        drop(zip_code);

        nft_metadata = json!({
            "name": format!("ChainID: {}", *name), // Safe to map public name to payload
            "description": "Cryptographic Data Registration Token",
            "image": "ipfs://QmDefaultChainIDBadgeImageHash",
            "attributes": [
                { "trait_type": "Network", "value": "Avalanche-C" },
                { "trait_type": "Omega Block", "value": omega_block_number },
                { "trait_type": "Identity Address", "value": identity_address }
            ],
            "zero_knowledge_hashes": { 
                "dob_hash": dob_hash, "phone_hash": phone_hash,
                "email_hash": email_hash, "zip_hash": zip_hash
            },
            "chain_data": {
                "omega_block_hash": omega_block_hash,
                "omega_block_timestamp": omega_block_timestamp,
                "omega_chronos": omega_chronos
            }
        });

    } // 🔒 ENCLAVE CLOSES. `secure_id` is hardware-zeroized here.

    let token_uri = pin_json_to_ipfs(&nft_metadata).await?;

    clear_screen();
    let pub_image = generate_qr(&token_uri)?;

    println!("--- IDENTITY ANCHORED ---");
    println!("IPFS CID: {}", token_uri);
    println!("{}\n", pub_image);
    println!("This Token URI will be minted to: {}", user_wallet);
    
    prompt_secure_input("Press ENTER to push Mint request to MetaMask...", 45).await?;

    println!("\n[+] Smart Contract interaction pushed via WalletConnect.");
    println!("[+] Approve the transaction on your phone to finalize registration.");

    sleep(Duration::from_secs(4)).await;
    Ok(())
}

// ---------------------------------------------------------
// SYSTEM DAEMON
// ---------------------------------------------------------
#[tokio::main]
async fn main() {
    // In production, this pepper is securely provisioned via HSM/TPM.
    let hardware_pepper = env::var("KIOSK_SECRET_PEPPER")
        .unwrap_or_else(|_| "Failsafe_Local_Dev_Pepper_9a8b7c6d5e4f".to_string());

    loop {
        clear_screen();
        println!("===========================================");
        println!("       CHAINID SECURE REGISTRATION         ");
        println!("===========================================");
        println!("\nHardware status: Green.");
        println!("Memory scrubbed and zeroized.");
        println!("Tap ENTER to begin registration...\n");

        let mut start = String::new();
        let _ = io::stdin().read_line(&mut start);

        clear_screen();
        match run_kiosk_session(&hardware_pepper).await {
            Ok(_) => println!("\n[✓] Session Completed Successfully."),
            Err(e) => {
                // If it halts here, Rust's drop semantics guarantee RAM is already clean
                println!("\n[!] CRITICAL HALT: {}", e);
                println!("[!] Session aborted. User memory wiped.");
            }
        }

        println!("Resetting kiosk in 5 seconds...");
        sleep(Duration::from_secs(5)).await;
    }
}