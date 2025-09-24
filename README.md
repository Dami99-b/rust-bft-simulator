# Rust BFT Simulator

A small experiment simulating Byzantine Fault Tolerance (BFT) in Rust.  
Shows how a group of nodes can still reach agreement even if some act maliciously.

## Features
- Honest vs. Byzantine nodes
- Simple vote + tally system
- Threshold check (2f+1 rule for safety)
- Multiple rounds with leader rotation

## Why
Distributed systems and blockchains rely on BFT consensus to stay secure under faults.  
This demo is a toy version to understand the basics.

## Example run
