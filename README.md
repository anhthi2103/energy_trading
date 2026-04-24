# Project Title

Energy Trading – A Soroban Smart Contract for Renewable Energy Credit Trading on Stellar

## Project Vision

This project enables **peer-to-peer trading of renewable energy credits** on the Stellar blockchain using Soroban smart contracts. Energy producers can list their surplus energy, and consumers can purchase credits directly, creating a transparent and efficient marketplace for renewable energy.

---

## Description

A Soroban smart contract dApp that facilitates **renewable energy credit trading** on Stellar Testnet. Producers list available energy units with pricing, and consumers purchase credits through a decentralized on-chain marketplace.

---

## Features

### 1. Energy Listing
- Energy producers can list surplus energy units for sale
- Each listing specifies amount and price per unit
- Persistent on-chain storage of all active listings

### 2. Direct Purchase
- Consumers can buy energy directly from listings
- Partial purchases supported (amount <= remaining units)
- Real-time remaining amount tracking

### 3. Transparent Marketplace
- All listings and transactions stored on-chain
- Anyone can query listing details and total supply
- Decentralized and censorship-resistant

### 4. Beginner-Friendly
- Clear, readable Rust code for Soroban
- Minimal, working example for learning

---

## Contract Functions

- **list_energy(producer, amount, price_per_unit)** — Create a new energy listing
- **buy_energy(consumer, listing_id, amount)** — Purchase energy from a listing
- **get_listing(listing_id)** — Get listing details (producer, amount, price, remaining)
- **get_total_listed()** — Get total energy units currently listed

---

## Contract

- **Contract ID**: [CCWM54IDRVRAUCJZCT6PHGEGFVZGLAVQPL4A5EFPPV5D4TPNXYYZGOLC](https://stellar.expert/explorer/testnet/tx/de4834e948e439ca8965b2bb13e4b9e7e4f408edf3ad5c519b9e78f9272f4158)

![screenshot](https://i.ibb.co/Nb6LtcT/image.png)

---

## Future Scopes

### 1. Escrow & Settlement
- Add multi-step escrow for secure energy trading

### 2. Energy Tokenization
- Tokenize energy production as tradable assets

### 3. Producer Ratings
- Add reputation system for energy producers

### 4. Dynamic Pricing
- Allow time-based or demand-based pricing adjustments

### 5. Batch Trading
- Enable bulk purchases across multiple listings

### 6. Frontend dApp
- Build a React web interface for easy trading

---

## Profile

- **Name:** anhthi2103
