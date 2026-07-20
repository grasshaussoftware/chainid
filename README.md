# ⛓️ ChainID

*(Based on the foundational blockchain ID demonstration shown [here](https://www.youtube.com/live/1iAg6BITPdc?si=oaGf4rTuJlCcQ5K8))*

> **Sovereign Digital Identity — Simple. Secure. Humane.**

A hardware-secured kiosk daemon that lets any person create a private, verifiable digital identity in under 60 seconds. ChainID leverages self-sovereign architecture to deliver immutable, portable, and indestructible civic credentials, facilitating secure identification checks and streamlining eligibility verification for voter ID and tax registration. 

Built with common sense first, designed for intuitive use, and rooted in the real-world need to help people who have lost everything.

---

## 📖 The Story

ChainID began in Wenatchee, Washington. Local businessman and former mayoral candidate James McLaughlin asked a simple question: **How do we help people experiencing homelessness register for basic services when they have no ID, no phone, and no fixed address?**

That question drove the architectural design and deployment of ChainID version 1.0 on the Avalanche blockchain. What started as a local inquiry has evolved into a dignified, privacy-first system that turns a brief kiosk visit into a permanent, sovereign digital identity anyone can prove without exposing their personal data.

---

## ⚡ The Problem vs. The Solution

**The Old Way:**
* Physical IDs get lost, stolen, or destroyed in the elements.
* Centralized government and corporate databases are honeypots vulnerable to massive breaches.
* People are forced to *borrow* their identity from the state.

**The ChainID Way:**
* Gives every human a clean, cryptographically verifiable credential they control completely. 
* Seamlessly scales from local community food bank check-ins to municipal voting booths.
* You *own* your identity. 

---

## 🚀 How It Works (A Frictionless Experience)

We built this to be completely frictionless for the end-user. 

1. **Approach:** Walk up to the kiosk — it wakes naturally.
2. **Scan:** Scan a single QR code with your Avalanche Core wallet.
3. **Verify:** Confirm your ID and face (utilizing passive liveness, no awkward poses required).
4. **Secure:** Enter any sensitive info privately directly on your own phone.
5. **Mint:** The kiosk hashes everything in a hardware secure enclave (TPM 2.0), wipes all raw data instantly, and mints a gasless NFT identity directly into your wallet.

You walk away with a beautiful, scannable digital credential. **No seed phrases. No maintenance. Just simple, private proof of who you are.**

---

## 🛠️ Under the Hood

* **Common-Sense UX:** Feels obvious and trustworthy the moment you interact with it.
* **Intuitive Elegance:** Smooth, frictionless interactions across the entire daemon.
* **Zero-Knowledge by Design:** Raw personal data is never stored—period.
* **Hardware-Grade Security:** TPM 2.0 coupled with full memory zeroization.
* **Gasless Minting:** Leverages ERC-4337 (the user pays absolutely nothing).
* **Permanent Storage:** Immutable data anchored on IPFS (InterPlanetary File System).
* **Native Avalanche Core Integration:** True seed abstraction means users never have to memorize seed phrases.
* **Grassroots Sensibility:** Built for the unhoused, the unbanked, and anyone who simply needs to prove who they are on their own terms.

---

## 💻 Quick Start

Get the daemon running locally in seconds:

```bash
git clone [https://github.com/grasshausmedia/chainid.git](https://github.com/grasshausmedia/chainid.git)
cd chainid
cargo run
```

[![Sovereign Digital Identity Demonstration](https://img.youtube.com/vi/1iAg6BITPdc/maxresdefault.jpg)](https://www.youtube.com/watch?v=1iAg6BITPdc)
