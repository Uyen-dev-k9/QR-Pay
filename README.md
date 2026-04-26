# Stellar QR Pay

Stellar QR Pay is a beginner-friendly Stellar Testnet payment project.

The project demonstrates a basic merchant payment flow where a user connects a Stellar wallet, checks their balance, sends a testnet payment, and sees the transaction result.

The long-term idea is to build a QR-based merchant payment app where users can scan a merchant QR code and choose to pay with fiat or stablecoins.

---

## Project Description

Stellar QR Pay is designed to simulate a simple merchant payment experience on Stellar.

In a real-world use case, a merchant can show a QR code to a customer. The customer scans the QR code, confirms the payment, and the merchant can verify whether the payment was completed.

For this Level 1 version, the project focuses on the basic Stellar payment flow:

- connect a Stellar Testnet wallet
- display the wallet balance
- submit a successful testnet transaction
- show the transaction result to the user

This project is the first step toward a larger QR payment system using Stellar.

---

## Features

- Connect Stellar Testnet wallet/account
- Display wallet/account balance
- Submit a testnet transaction
- Show transaction result to the user
- Demonstrate a simple merchant payment flow

---

## Built With

- Stellar Testnet
- Soroban Studio
- Rust
- Stellar Testnet account
- Stellar transaction explorer

---

## Setup Instructions

This project was built and tested using **Soroban Studio**.

### 1. Open Soroban Studio

Go to:

```txt
https://soroban.studio/
```

### 2. Create or Open the Project

Create a new Stellar/Soroban project and open the contract file.

### 3. Add the Contract Code

Open the `lib.rs` file and paste the contract code.

### 4. Build the Contract

Click:

```txt
Build Contract
```

Wait until the build is successful.

### 5. Deploy to Stellar Testnet

After the contract is built successfully, click:

```txt
Deploy
```

Select:

```txt
Stellar Testnet
```

After deployment, copy the Contract ID.

### 6. Interact With the Contract

Use the function panel in Soroban Studio to call the contract functions.

Example functions used in this project:

```txt
create_payment()
get_payment()
pay()
is_paid()
```

### 7. Check the Result

After calling the payment function, the transaction result is displayed in Soroban Studio.

The user can see:

- transaction submitted successfully
- returned result
- payment status
- emitted event

---

## Testnet Contract Information

**Project Name:** Stellar QR Pay

**Network:** Stellar Testnet

**Contract ID:**

```txt
CB6XBW7BQBEBBNL2GVCB5FKZ6YG4Z3RFYA3UBULFQDPP6V4UXHASXZSO
```

---

## Screenshots


### 1. Wallet Connected State

<img width="665" height="558" alt="image" src="https://github.com/user-attachments/assets/5cd5ec58-9e00-44e4-8173-8b480522028d" />

### 2. Successful Testnet Transaction

<img width="1280" height="242" alt="image" src="https://github.com/user-attachments/assets/6fbca9a6-6a90-49db-b6d0-e2fd6a539f25" />

### 3. Transaction Result Shown to the User

<img width="1280" height="225" alt="image" src="https://github.com/user-attachments/assets/b2bb13c7-fc19-4dcb-9d76-78432265c84b" />

---

## Author

Built by: `your-name-here`

Project: Stellar QR Pay

Network: Stellar Testnet
