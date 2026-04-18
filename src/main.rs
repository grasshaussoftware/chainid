use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use qrcode::QrCode;
use qrcode::render::unicode;
use serde_json::json;
use std::io::{self, Write};
use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::mpsc;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

// Elegant, minimalist Apple-inspired ANSI styling
const CLEAR: &str = "\x1b[2J\x1b[1;1H";
const RESET: &str = "\x1b[0m";
const SOFT_GREEN: &str = "\x1b[38;5;10m";
const SOFT_GRAY: &str = "\x1b[38;5;245m";
const BOLD: &str = "\x1b[1m";

fn clear_screen() {
    print!("{}", CLEAR);
    let _ = io::stdout().flush();
}

async fn type_text(text: &str, speed_ms: u64) {
    print!("{}{}", SOFT_GREEN, BOLD);
    for c in text.chars() {
        print!("{}", c);
        let _ = io::stdout().flush();
        sleep(Duration::from_millis(speed_ms)).await;
    }
    println!("{}", RESET);
}

fn generate_qr(data: &str) -> String {
    let code = QrCode::new(data).unwrap();
    code.render::<unicode::Dense1x2>().quiet_zone(false).build()
}

async fn secure_hash_pii(data: &str, salt: &str, pepper: &str) -> String {
    let key = format!("{}{}", salt, pepper);
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

async fn pin_to_ipfs(_metadata: &serde_json::Value) -> String {
    // In production this would call a real IPFS node
    sleep(Duration::from_secs(1)).await;
    "ipfs://QmChainIDProofPlaceholderForDemo".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen();
    println!("{}ChainID{}", SOFT_GRAY, RESET);
    println!("Sovereign Digital Identity\n");

    type_text("Connecting to Avalanche C-Chain...", 15).await;
    let provider = Provider::<Http>::try_from("https://api.avax.network/ext/bc/C/rpc")?;
    let block = provider.get_block(BlockNumber::Latest).await?.unwrap();
    let block_num = block.number.unwrap();

    println!("{}System secure • Block {} • Ready{}", SOFT_GRAY, block_num, RESET);
    println!();

    let (tx, mut rx) = mpsc::channel::<String>(32);
    let mut user_wallet = String::new();

    // Simulated proximity detection
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        let _ = tx_clone.send("proximity".to_string()).await;
    });

    while let Some(event) = rx.recv().await {
        match event.as_str() {
            "proximity" => {
                type_text("Welcome. Scan the QR code with your Avalanche Core app.", 20).await;
                let wc_uri = "wc:demo-chainid-kiosk-connection-2026";
                println!("\n{}", generate_qr(wc_uri));
                println!();

                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    sleep(Duration::from_secs(3)).await;
                    let _ = tx_clone.send("connected:0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B".to_string()).await;
                });
            }
            s if s.starts_with("connected:") => {
                user_wallet = s.strip_prefix("connected:").unwrap().to_string();
                clear_screen();
                type_text(&format!("Connected securely to {}", user_wallet), 15).await;
                type_text("For your privacy, please enter SSN on your phone.", 20).await;

                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    sleep(Duration::from_secs(2)).await;
                    let _ = tx_clone.send("ssn_received".to_string()).await;
                });
            }
            "ssn_received" => {
                type_text("ID scan requested. Place your ID and look at camera.", 20).await;

                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    sleep(Duration::from_secs(3)).await;
                    let _ = tx_clone.send("biometrics_ok".to_string()).await;
                });
            }
            "biometrics_ok" => {
                type_text("✓ Identity verified. Processing in secure enclave...", 20).await;
                
                let omega_hash = format!("{:#x}", block.hash.unwrap());
                let hardware_pepper = "apple_secure_enclave_pepper_demo_2026";
                
                let dob_hash = secure_hash_pii("1990-01-01", &omega_hash, hardware_pepper).await;
                let zip_hash = secure_hash_pii("98801", &omega_hash, hardware_pepper).await;
                let ssn_hash = secure_hash_pii("000-00-0000", &omega_hash, hardware_pepper).await;

                let metadata = json!({
                    "name": "ChainID Sovereign Identity",
                    "description": "Zero-knowledge proof of verified human identity",
                    "attributes": [
                        {"trait_type": "Network", "value": "Avalanche C-Chain"},
                        {"trait_type": "Block", "value": block_num.as_u64()},
                        {"trait_type": "Wallet", "value": user_wallet}
                    ],
                    "zero_knowledge_hashes": {
                        "dob": dob_hash,
                        "zip": zip_hash,
                        "ssn": ssn_hash
                    }
                });

                let ipfs_cid = pin_to_ipfs(&metadata).await;
                type_text(&format!("✓ Anchored to IPFS: {}", ipfs_cid), 20).await;
                type_text("Gasless mint in progress via ERC-4337...", 20).await;
                
                sleep(Duration::from_secs(2)).await;
                type_text("✓ ChainID minted successfully to your Core wallet.", 20).await;
                type_text("You are all set. Thank you for choosing dignity and privacy.", 25).await;
                
                sleep(Duration::from_secs(5)).await;
                clear_screen();
                println!("{}ChainID complete. System ready for the next person.{}", SOFT_GRAY, RESET);
            }
            _ => {}
        }
    }
    Ok(())
}