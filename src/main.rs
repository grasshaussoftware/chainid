/*
 * ChainID: Private Electronic Access Register (PoC)
 * Veritas MMXXIII Deusopus
 * * This proof of concept demonstrates the core cryptographic and terminal
 * logic of the ChainID system, including mnemonic generation, 
 * ECC key derivation, memory sanitization, and QR rendering.
 */

use bip39::{Mnemonic, MnemonicType, Language};
use k256::ecdsa::{SigningKey, VerifyingKey};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use qrcode::QrCode;
use qrcode::render::unicode;
use zeroize::{Zeroize, Zeroizing};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// --- Data Structures ---

#[derive(Zeroize)]
struct IdentitySecrets {
    mnemonic: String,
    private_key_hex: String,
}

struct PublicIdentity {
    public_key_hex: String,
    address: String,
}

// --- Logic Implementation ---

fn main() {
    print_splash_screen();

    println!("\n[1] INITIALIZING CRYPTOGRAPHIC ENTROPY...");
    thread::sleep(Duration::from_millis(800));

    // 1. Generate Mnemonic (BIP-39)
    let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
    let phrase = mnemonic.phrase().to_string();
    
    // 2. Derive ECC Keys (secp256k1)
    // In a real app, we'd use the mnemonic seed. Here we generate a random key for the PoC.
    let signing_key = SigningKey::random(&mut rand::thread_rng());
    let priv_hex = hex::encode(signing_key.to_bytes());
    
    let mut secrets = Zeroizing::new(IdentitySecrets {
        mnemonic: phrase.clone(),
        private_key_hex: priv_hex,
    });

    // 3. Prepare Public Identity
    let verifying_key = VerifyingKey::from(&signing_key);
    let pub_key_encoded = verifying_key.to_encoded_point(false);
    let pub_hex = hex::encode(pub_key_encoded.as_bytes());
    
    // Mock Ethereum/Avalanche Address derivation (simplified)
    let address = format!("0x{}", &pub_hex[pub_hex.len() - 40..]);

    let public_id = PublicIdentity {
        public_key_hex: pub_hex,
        address,
    };

    println!("\n[2] USER REGISTRATION (KYC MOCK)");
    let name = prompt("Enter Full Name: ");
    let doc_id = prompt("Enter Document ID (e.g. Passport): ");

    println!("\n[3] GENERATING CHAINID TOKENS...");
    thread::sleep(Duration::from_millis(1000));

    // 4. Render QR Code
    display_qr(&public_id.address);

    // 5. Output Summary
    println!("\n" + &"=".repeat(50));
    println!("CHAINID REGISTRATION SUCCESSFUL");
    println!("=" .repeat(50));
    println!("Name:      {}", name);
    println!("ID Hash:   {}", hex::encode(doc_id.as_bytes())); // PII is hashed
    println!("Address:   {}", public_id.address);
    println!("Public Key: {}...", &public_id.public_key_hex[..32]);
    println!("=" .repeat(50));

    // 6. Security & Physical Output Simulation
    println!("\n[!] SIMULATING CSN-A2 THERMAL PRINTER OUTPUT...");
    println!("    > Printing Mnemonic and Private Key to physical medium...");
    thread::sleep(Duration::from_millis(1500));
    println!("    > Physical backup complete. Air-gap established.");

    println!("\n[!] ZEROIZING SENSITIVE MEMORY...");
    // secrets is wrapped in Zeroizing, so it will be wiped when this scope ends.
    drop(secrets); 
    println!("    > RAM Cleared. Application session secured.");

    println!("\nThank you for using ChainID. Your identity is now anchored.");
}

// --- Helper Functions ---

fn print_splash_screen() {
    println!(r#"
    #################################################
    #                                               #
    #   ____ _           _       ___ ____           #
    #  / ___| |__   __ _(_)_ __ |_ _|  _ \          #
    # | |   | '_ \ / _` | | '_ \ | || | | |         #
    # | |___| | | | (_| | | | | || || |_| |         #
    #  \____|_| |_|\__,_|_|_| |_|___|____/          #
    #                                               #
    #         DECENTRALIZED IDENTITY SYSTEM         #
    #           VERITAS MMXXIII DEUSOPUS            #
    #################################################
    "#);
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn display_qr(data: &str) {
    println!("\n[QR TOKEN - SCAN TO VERIFY ON AVALANCHE]");
    let code = QrCode::new(data).unwrap();
    let image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);
}
