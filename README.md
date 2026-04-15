
# ChainID Kiosk Protocol 🛡️

**A Zero-Trust, Hardware-Secured Web3 Identity Registration Terminal**

ChainID is an enterprise-grade, physical kiosk software written in Rust. It bridges the gap between off-chain physical identity (KYC data) and on-chain decentralized identity (Avalanche-C NFTs and IPFS) without ever exposing the user's private keys or plaintext personally identifiable information (PII) to the public ledger.

By utilizing hardware-level memory scrubbing, WalletConnect, and Zero-Knowledge HMAC hashing, ChainID provides a secure "coat check" system for global identity.

---

## ✨ Core Architecture & Security Features

* **Air-Gapped Private Keys:** Cryptographic seed phrases (BIP-39) are generated in a secure enclave and dispatched directly to a physical thermal printer. The private key is *never* displayed on the screen.
* **Hardware-Level RAM Scrubbing:** Utilizes the [`zeroize`](https://crates.io/crates/zeroize) crate to cryptographically overwrite volatile memory (RAM) with zeroes the exact millisecond the session concludes, times out, or crashes.
* **Hardware-Peppered PII Hashing:** Protects low-entropy user data (like Date of Birth or ZIP codes) from brute-force/rainbow table attacks by hashing the data using `HMAC-SHA256` combined with a public blockchain salt AND a private hardware TPM pepper.
* **Zero-Fund Kiosk (WalletConnect):** The terminal itself holds no cryptocurrency and cannot be drained. Users authenticate and pay for the NFT minting gas fees directly from their own mobile devices via MetaMask.
* **Aggressive Idle Protection:** Strict asynchronous timeouts drop the connection and wipe memory if the user walks away from the kiosk at any point during registration.
* **Interplanetary Ready (IPFS):** Public metadata is pinned to the decentralized web, ensuring identity verifiable records survive independently of centralized servers.

## 🛠️ Tech Stack

* **Language:** Rust (Edition 2021)
* **Blockchain:** `ethers-rs` (Avalanche-C EVM compatible)
* **Cryptography:** `hmac`, `sha2`, `coins-bip39`
* **Memory Safety:** `zeroize`
* **Asynchronous Runtime:** `tokio`
* **Network:** `reqwest` (IPFS pinning)

## 🖨️ Physical Hardware Requirements

To deploy this kiosk software in a production environment, the following hardware is required:
1.  **Touchscreen Interface:** With randomized on-screen keyboard software to prevent thermal/fingerprint tracking. (No physical keyboards).
2.  **Thermal Receipt Printer:** Configured to receive direct serial/USB dispatch from the Rust backend.
3.  **Hardware Secure Enclave (TPM/HSM):** To securely store the `KIOSK_SECRET_PEPPER` environment variable.

## 🚀 Installation & Deployment

### 1. Prerequisites
Ensure you have the latest stable version of Rust installed:
```
```text?code_stdout&code_event_index=2
README.md generated

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

### 2. Clone the Repository
```bash
git clone [https://github.com/grasshaussoftware/cryptographic-data-registration.git](https://github.com/grasshaussoftware/cryptographic-data-registration.git)
cd cryptographic-data-registration
```

### 3. Build for Production
**CRITICAL:** You must build the application in release mode. Rust's compiler optimizations in release mode are required to guarantee the hardware-level memory scrubbing operates as designed.
```bash
cargo build --release
```

### 4. Run the Kiosk Daemon
Provision the kiosk with your secure hardware pepper and start the terminal attract loop:
```bash
export KIOSK_SECRET_PEPPER="<YOUR_SECURE_HARDWARE_PEPPER>"
./target/release/chainid-kiosk
```

## 🔄 The User Journey Flow

1.  **Attract Loop:** Terminal displays a secure, idle state while continuously scrubbing memory.
2.  **Authentication:** User scans a rotating WalletConnect QR code with their mobile Web3 wallet.
3.  **Data Entry:** User enters physical KYC data (Name, DOB, Phone, Email, ZIP) on the touchscreen.
4.  **Hardware Generation:** A secure enclave generates a 24-word seed phrase and derives an EVM address. The terminal prints the private seed to the physical thermal receipt.
5.  **Zero-Knowledge Anchoring:** The kiosk fetches the latest Avalanche-C block hash, combines it with the hardware pepper, and HMAC-hashes the user's PII. 
6.  **Zeroize:** The RAM holding the seed phrase and plaintext PII is instantly overwritten with zeroes.
7.  **IPFS & Mint:** The hashed payload is pinned to IPFS. A transaction request is pushed back to the user's phone to mint the public NFT identity token.

---

### 📜 License & Contact
**Author:** grasshaussoftware  
**Contact:** deusopus@duck.com  
*(c) 2023 - 2026 all rights reserved.*