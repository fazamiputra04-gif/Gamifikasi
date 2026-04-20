# Stellar QuestNotes DApp

**Stellar QuestNotes DApp** - Gamified & Decentralized Note-Taking System on the Blockchain

## Project Description

Stellar QuestNotes DApp is an evolution of traditional note-taking, built as a decentralized smart contract on the Stellar blockchain using the Soroban SDK. It merges personal productivity with **Gamification**, providing a secure and immutable platform where users are rewarded for their contributions. 

By leveraging Stellar's fast and low-cost network, the contract ensures that your data is stored transparently. Each note is cryptographically bound to your wallet address, and every interaction earns you **Experience Points (XP)** to increase your on-chain **Level**, eliminating reliance on centralized databases while keeping note-taking engaging.

## Project Vision

Our vision is to revolutionize personal productivity in the digital age by:

- **Incentivizing Productivity**: Rewarding users with an automated XP and Leveling system to encourage consistent documentation.
- **Ensuring Sovereign Ownership**: Empowering users with complete control over their digital thoughts using cryptographic authentication (`require_auth`).
- **Guaranteeing Immutability**: Providing a permanent, tamper-proof record of notes and user achievements.
- **Building Trustless Systems**: Creating a platform where data integrity, XP calculation, and leveling up are guaranteed by code, without third-party intervention.

## Key Features

### 1. **Gamified Note Creation**
- Earn **+10 XP** for every note created.
- Automated level-up system (users level up every 50 XP).
- Contract returns dynamic feedback (e.g., "Level Up!" notifications) upon successful transactions.

### 2. **Decentralized User Profiles**
- Dedicated on-chain profiles tracking each user's XP, current Level, and total notes created.
- Profile data is securely stored using Soroban's **Persistent Storage** to ensure long-term progress retention.

### 3. **Secure & Authenticated Interactions**
- Utilizes Soroban's native `require_auth()` to ensure only the true owner of a wallet address can create notes or earn XP under their identity.
- Protected against unauthorized modifications or identity spoofing.

### 4. **Efficient Data Retrieval**
- Fetch your gamification profile instantly using your address.
- Retrieve all stored notes across the network in a single call.
- Uses **Instance Storage** for the global note collection to optimize contract state management.

### 5. **Stellar Network Integration**
- Built using the modern Soroban Smart Contract SDK (Rust).
- Leverages the high speed and low cost of the Stellar ecosystem.

## Contract Details

- **Contract Address**: CB6B3RYTFSBGXVT56HQRPQLYO6WQEBORB22CJDP23FTL5LZQ7HY2IU3N
- **Language**: Rust (`#![no_std]`)
- **Network**: Stellar Soroban (Testnet/Mainnet)

## Future Scope

### Short-Term Enhancements
1. **Global Leaderboards**: Display the top 10 users with the highest levels on the network.
2. **Streak Bonuses**: Award multiplier XP for creating notes on consecutive days.
3. **Category Management**: Add tags and categories to organize notes efficiently.

### Medium-Term Development
4. **Achievement Badges**: Mint specific NFTs automatically when a user reaches milestone levels (e.g., Level 10, Level 50).
5. **Note Encryption**: Support for end-to-end encryption of note content using public keys for enhanced privacy.
6. **Secure Deletion**: Allow users to safely delete or archive their notes while retaining their earned XP.

### Long-Term Vision
7. **DAO Governance**: Community-driven protocol improvements where high-level users gain voting power.
8. **Decentralized UI Hosting**: Host the frontend on IPFS or similar decentralized platforms.
9. **Asset Attachment**: Capability to attach digital assets or tokens to specific notes or reward pools.

---

## Technical Requirements

- Soroban SDK
- Rust programming language (`cargo`, `rustc`)
- Stellar CLI

## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using the three main entry points:

- `Notes(user: Address, title: String, content: String)` - Authenticate, create a note, and earn +10 XP.
- `get_profile(user: Address)` - Retrieve a user's current gamification stats (XP, Level, Notes Count).
- `get_all_notes()` - Fetch the entire collection of notes stored in the contract.

---

**Stellar QuestNotes DApp** - Level Up Your Thoughts on the Blockchain.