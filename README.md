# HelPet - Solana Smart Contracts

This repository contains the smart contracts for the HelPet game ecosystem on Solana. The system consists of three main contracts that work together to create an engaging pet collection and rewards system.

## Smart Contracts Overview

### 1. HelPet Token (helpet_token)
- Implements the HPET token using the SPL Token standard
- HPET is the main utility token used throughout the game ecosystem
- Players earn HPET by collecting bones and can spend it to unlock animals

### 2. HelPet Rewards (helpet_rewards) 
- Handles the reward distribution system
- Main functions:
  - `collect_bones`: Mints HPET tokens to players when they collect bones in the game
  - `unlock_animal`: Processes token payments when players unlock new animals
- Interacts with the HPET token contract to mint and transfer tokens

### 3. HelPet Unlock (helpet_unlock)
- Manages the animal unlocking system
- Main functions:
  - `unlock_animal`: Verifies token balance and processes the unlocking of new animals
  - `spend_tokens`: Handles token spending for additional features
- Requires 100 HPET tokens to unlock a new animal
- Maintains state of unlocked animals per player

## Contract Interactions

1. Players collect bones in the game → `helpet_rewards` mints HPET tokens as rewards
2. Players accumulate HPET tokens in their wallet
3. Players can spend HPET tokens through `helpet_unlock` to:
   - Unlock new animals (100 HPET)
   - Access additional features (variable amounts)

## Technical Requirements

- Solana Program Library (SPL) Token v3.3.0
- Solana Program Runtime v1.18
- All contracts are written in Rust

## Account Structure

Each contract requires specific accounts for operations:

### For Rewards:
- Player's account
- HPET mint account
- Player's token account
- SPL Token program

### For Unlocking:
- Player's account
- Player's HPET token account
- SPL Token program
- Animal state account

## Security Considerations

- Token operations are protected by Solana's native security features
- Balance checks are performed before any token transfers
- Program-derived addresses are used where necessary
- All critical operations require proper account ownership verification

## Getting Started

1. Install Solana CLI tools
2. Clone the repository
3. Build each contract using `cargo build-bpf`
4. Deploy contracts to Solana network
5. Initialize HPET token mint
6. Configure program IDs in your client application
