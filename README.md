# Gig-Pay India / Freelancer-Bridge

**Get paid by your international clients in crypto. Receive it as Rupees in your bank account, instantly.**

---

## Table of Contents

- [Project Overview](#project-overview)
- [Problem Statement](#problem-statement)
- [Solution](#solution)
- [Features](#features)
- [Technology Stack](#technology-stack)
- [How It Works](#how-it-works)
- [Getting Started](#getting-started)
- [License](#license)
- [Contact](#contact)

---

## Project Overview

Gig-Pay India (also called Freelancer-Bridge) is a CLI-based platform designed for Indian gig workers—developers, designers, writers, and freelancers—to receive payments from international clients quickly and securely. The platform accepts USDC crypto payments from clients and simulates converting them into INR, which is then "pushed" to a freelancer's UPI ID for instant bank payout.

This project is a **prototype** demonstrating the flow of a freelancer payout system, using Rust CLI, Stellar testnet, and simulated bank payouts.

---

## Problem Statement

Freelancers in India face several issues when working with international clients:

- Delayed payments due to banking processes  
- High transaction fees for international transfers  
- Complicated currency conversion processes  
- Lack of transparency and tracking  

---

## Solution

Gig-Pay India provides a seamless payout system that allows freelancers to:

- Receive crypto payments from clients worldwide  
- Instantly convert crypto to INR (simulated in the prototype)  
- Withdraw money directly to their bank accounts via UPI  
- Track all transactions via CLI output  

This ensures faster payments, lower fees, and hassle-free international freelancing.

---

## Features

- **CLI Interface:** Simple command-line flow for wallet input, balance check, and withdrawal.  
- **Stellar Integration:** Uses `stellar-cli` to check USDC balances on the testnet.  
- **Instant INR Payout:** Simulates sending INR to a freelancer's UPI ID.  
- **Prototype Safety:** Commands are displayed for security; no private keys are stored.  
- **Step-by-Step Guidance:** The user is guided through wallet, balance, amount, and payout confirmation.  

---

## Technology Stack

- **Language:** Rust  
- **Crypto Integration:** Stellar testnet via `stellar-cli`  
- **Bank/Payment Simulation:** Rust functions simulating UPI payouts  
- **Development Tools:** `cargo`, Rust standard libraries (`std::io`, `std::process`)  
- **Prototype Focus:** CLI-based flow for proof-of-concept  

---

## How It Works

1. User runs the Rust CLI application.  
2. User inputs their Stellar wallet address.  
3. The CLI queries the Stellar testnet using `stellar-cli` to fetch the USDC balance (simulated if CLI fails).  
4. User specifies how much USDC to withdraw.  
5. User inputs their UPI ID.  
6. The app calculates INR payout using a simulated USDC → INR exchange rate.  
7. CLI displays a review screen with USDC amount, exchange rate, INR payout, and UPI ID.  
8. Upon confirmation:  
   - Shows the Stellar transaction command to "take" the USDC (prototype simulation).  
   - Simulates sending INR to the user's UPI ID.  
9. Updates the CLI with the new wallet balance and payout success message.  

---

## Getting Started

1. **Build the application:**

   `cargo build --release`

3. **Run the CLI prototype:**

   `cargo run`

4. **Follow the prompts:**

   Enter your Stellar wallet address

   Check balance

   Enter withdrawal amount in USDC

   Enter UPI ID

   Confirm transaction

   CLI will simulate USDC capture and INR payout

---

## License

This project is licensed under the MIT License. See the LICENSE file for details.



---

## Contact

Project Lead: Frahan Alam

Email: b25135@students.iitmandi.ac.in

GitHub: https://github.com/Frozen-afk
