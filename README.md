# Solana On-Chain Escrow (PDA)

A professional implementation of a trustless escrow service on the Solana blockchain. This repository demonstrates how to use Program Derived Addresses (PDAs) to act as autonomous "vaults" for SPL tokens and SOL, removing the need for a centralized third party.

## Key Concepts
* **PDAs (Program Derived Addresses)**: Accounts controlled solely by the program, used here to hold escrowed funds.
* **Cross-Program Invocations (CPI)**: Interacting with the SPL Token Program to transfer assets securely.
* **State Management**: Professional serialization of escrow states including Maker, Taker, and Expected Amount.

## Features
* **Initialize**: Maker deposits tokens into a PDA vault and specifies the exchange rate.
* **Exchange**: Taker sends the requested tokens to the Maker, triggering the vault to release funds to the Taker.
* **Cancel**: Maker can reclaim funds before the exchange is completed.

## Structure
This is a flat-structure repository designed for clarity and easy integration into Anchor or native Rust frameworks.
