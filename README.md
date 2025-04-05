# Solana Pyth CPI Example

A secure demonstration of Cross-Program Invocations (CPIs) between Solana programs to fetch and process Pyth price feed data using the Anchor framework.

## Overview

This project demonstrates how to securely implement Cross-Program Invocations (CPIs) on Solana by building a price fetcher program that communicates with an oracle program to retrieve price data from Pyth price feeds. The implementation includes robust security measures to prevent reentrancy vulnerabilities.

## Features

### Price Fetcher Program

- Securely invokes the Oracle program to fetch price data
- Validates input parameters like feed ID and maximum age
- Implements reentrancy protection through pre/post-CPI validation
- Handles errors gracefully with custom error types
- Performs comprehensive account validation

### Oracle Program

- Interfaces directly with Pyth price feeds
- Validates price feed parameters
- Retrieves up-to-date price data with confidence intervals
- Enforces freshness through maximum age parameter
- Returns formatted price data with proper scaling

### Security Features

- Reentrancy protection with state validation before and after CPIs
- Follows the "checks-effects-interactions" pattern for secure external calls
- Proper account ownership verification
- Comprehensive validation of feed IDs and parameters
- Detection of unexpected account state modifications

## Technical Details

- Built with Anchor framework for Solana
- Implements Cross-Program Invocations (CPIs) securely
- Integrates with Pyth price oracle network
- Uses strong typing for account validation
- Features comprehensive error handling and state validation

## Usage

The program demonstrates how to:
1. Define program interfaces for secure CPI
2. Implement proper validation before and after external calls
3. Handle external program results and errors
4. Protect against reentrancy vulnerabilities
5. Fetch and process real-time price data from Pyth oracles

This example serves as a reference implementation for developers building secure Solana programs that need to interact with external programs and oracles. 
