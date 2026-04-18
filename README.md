# ChainID

**Sovereign Digital Identity — Simple. Secure. Humane.**

A hardware-secured kiosk daemon that lets any person create a private, verifiable digital identity in under 60 seconds. Built with common sense first, designed like an Apple product, and rooted in the real-world need to help people who have lost everything.

---

### The Story
ChainID began in Wenatchee, Washington. Local businessman and former mayoral candidate James McLaughlin asked a simple question: *How do we help people experiencing homelessness register for basic services when they have no ID, no phone, and no fixed address?*

That question became ChainID — a dignified, privacy-first system that turns a brief kiosk visit into a permanent, sovereign digital identity anyone can prove without exposing their personal data.

---

### Why ChainID Exists
- Physical IDs get lost, stolen, or destroyed.
- Centralized databases are vulnerable to breaches.
- People should own their identity — not borrow it from governments or corporations.

ChainID gives every human a clean, cryptographically verifiable credential they control completely.

---

### How It Works (The Apple-Level Experience)
1. Walk up to the kiosk — it wakes naturally.
2. Scan one QR code with your Avalanche Core wallet.
3. Verify your ID and face (passive liveness, no awkward poses).
4. Enter sensitive info privately on your own phone.
5. The kiosk hashes everything in a hardware secure enclave (TPM 2.0), wipes all raw data instantly, and mints a gasless NFT identity directly into your wallet.

You walk away with a beautiful, scannable digital credential. No seed phrases. No maintenance. Just simple, private proof of who you are.

---

### Key Features
- **Common-sense-first UX** — Feels obvious and trustworthy the moment you see it
- **Apple-level elegance** in every interaction
- **Zero-knowledge by design** — raw personal data is never stored
- **Hardware security** — TPM 2.0 + full memory zeroization
- **Gasless minting** via ERC-4337 (you pay nothing)
- **Permanent storage** on IPFS (InterPlanetary File System)
- **Native Avalanche Core wallet integration** (seed abstraction — no seed phrases needed)
- **Grassroots sensibility** — built for the unhoused, the unbanked, and anyone who simply needs to prove who they are

---

### Quick Start

```bash
git clone https://github.com/grasshaussoftware/chainid.git
cd chainid
cargo run