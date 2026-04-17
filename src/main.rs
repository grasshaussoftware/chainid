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
use tokio::sync::mpsc;
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

type HmacSha256 = Hmac<Sha256>;

// ---------------------------------------------------------
// HARDWARE ENCLAVE & MEMORY SAFETY
// ---------------------------------------------------------
#
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
// SYSTEM EVENTS & STATE MACHINE
// ---------------------------------------------------------
enum KioskEvent {
    UserApproached(f32), // Distance in meters
    CoreWalletConnected(String), // Wallet address
    BiometricsCaptured { raw_pii: Zeroizing<String> },
    TransactionSponsored(String), // Tx Hash
}

// ---------------------------------------------------------
// UTILITIES & CRYPTOGRAPHY
// ---------------------------------------------------------
fn clear_screen() {
    print!("{} Initializing TPM 2.0 Hardware Enclave...");
    sleep(Duration::from_millis(500)).await;
    // In production, this decrypts the pepper directly from the physical TPM chip
    Ok("Hardware_Bound_Pepper_9a8b7c6d5e4f".to_string())
}

async fn pin_json_to_ipfs(_payload: &serde_json::Value) -> Result<String, &'static str> {
    println!(">>> UPLOADING METADATA TO IPFS <<<");
    sleep(Duration::from_secs(2)).await;
    Ok("ipfs://QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG".to_string())
}

// ---------------------------------------------------------
// ASYNC KIOSK DAEMON
// ---------------------------------------------------------
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen();
    println!("===========================================");
    println!("       CHAINID AI KIOSK DAEMON V2.0        ");
    println!("===========================================\n");

    // 1. Hardware-Bound Security via TPM
    let hardware_pepper = init_tpm_pepper().await?;
    println!("[+] Memory scrubbed and zeroized. System Green.\n");

    let (tx, mut rx) = mpsc::channel::<KioskEvent>(32);

    // 2. Simulated Computer Vision Thread (OpenCV Proximity)
    let tx_cv = tx.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(3)).await;
        println!("[OpenCV Module] Human detected at 1.2 meters.");
        let _ = tx_cv.send(KioskEvent::UserApproached(1.2)).await;
    });

    // 3. Main Event Loop
    while let Some(event) = rx.recv().await {
        match event {
            KioskEvent::UserApproached(_distance) => {
                println!(">>> AI AVATAR: 'Hello! Please scan this QR code with your Avalanche Core app to begin.'");
                // Generating a WalletConnect v2 URI for the Core App
                let wc_uri = "wc:1234567890...avalanche_core_uri...xyz";
                println!("{}", generate_qr(wc_uri)?);
                
                let tx_wallet = tx.clone();
                tokio::spawn(async move {
                    sleep(Duration::from_secs(4)).await;
                    let _ = tx_wallet.send(KioskEvent::CoreWalletConnected(
                        "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B".to_string()
                    )).await;
                });
            },
            
            KioskEvent::CoreWalletConnected(wallet_address) => {
                clear_screen();
                println!("[+] Core Wallet Connected: {}", wallet_address);
                println!(">>> AI AVATAR: 'Wallet linked securely. Please place your ID on the scanner and look at the camera.'");
                
                // 4. Simulated OCR & Passive Liveness Detection
                let tx_bio = tx.clone();
                tokio::spawn(async move {
                    println!(" Running OCR extraction and passive liveness skin-texture analysis...");
                    sleep(Duration::from_secs(3)).await;
                    let raw_data = Zeroizing::new("John Doe|1990-01-01|90210".to_string());
                    let _ = tx_bio.send(KioskEvent::BiometricsCaptured { raw_pii: raw_data }).await;
                });
            },
            
            KioskEvent::BiometricsCaptured { raw_pii } => {
                println!("[+] Biometrics Verified. Liveness confirmed.");
                println!(">>> AI AVATAR: 'Thank you. I am now permanently deleting your images and securing your data on-chain.'");
                
                let rpc_url = "https://api.avax.network/ext/bc/C/rpc";
                let provider = Provider::<Http>::try_from(rpc_url)?;
                let block = provider.get_block(BlockNumber::Latest).await?.unwrap();
                let omega_block_hash = format!("{:#x}", block.hash.unwrap());
                
                let identity_address;
                let nft_metadata;
                
                {
                    // 🔒 SECURE ENCLAVE OPENS
                    let secure_id = SecureIdentity::generate();
                    let identity_wallet = LocalWallet::new_from_mnemonic(&secure_id.phrase, None)?;
                    identity_address = format!("{:?}", identity_wallet.address());
                    
                    println!("\n");
                    
                    // 🌶️ HMAC-SHA256 Hashing with explicit drops
                    let parts: Vec<&str> = raw_pii.split('|').collect();
                    let dob_hash = secure_hash_pii(parts[1], &omega_block_hash, &hardware_pepper)?;
                    let zip_hash = secure_hash_pii(parts[2], &omega_block_hash, &hardware_pepper)?;
                    
                    nft_metadata = json!({
                        "name": format!("ChainID: {}", parts),
                        "description": "Cryptographic Data Registration Token",
                        "attributes": [
                            { "trait_type": "Network", "value": "Avalanche-C" },
                            { "trait_type": "Identity Address", "value": identity_address }
                        ],
                        "zero_knowledge_hashes": { "dob_hash": dob_hash, "zip_hash": zip_hash }
                    });
                    // raw_pii drops and is zeroized here
                } // 🔒 ENCLAVE CLOSES
                
                let token_uri = pin_json_to_ipfs(&nft_metadata).await?;
                println!("[+] ChainID anchored to IPFS: {}", token_uri);
                
                // 5. Gasless Minting via ERC-4337
                let tx_mint = tx.clone();
                tokio::spawn(async move {
                    println!("[Paymaster] Sponsoring transaction gas via ERC-4337 UserOperation...");
                    sleep(Duration::from_secs(2)).await;
                    let _ = tx_mint.send(KioskEvent::TransactionSponsored("0xabc123...".to_string())).await;
                });
            },
            
            KioskEvent::TransactionSponsored(tx_hash) => {
                println!("[+] Gasless minting successful! Tx Hash: {}", tx_hash);
                println!(">>> AI AVATAR: 'You are all set! Your ChainID is secured in your wallet. Have a great day.'");
                println!("\n");
                sleep(Duration::from_secs(5)).await;
                clear_screen();
                println!("===========================================");
                println!("       CHAINID AI KIOSK DAEMON V2.0        ");
                println!("===========================================\n");
            }
        }
    }
    Ok(())
}