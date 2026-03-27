# 🚀 Soroban Counter Smart Contract

## 📌 Project Description

This project is a basic smart contract built using Soroban on the Stellar network. It demonstrates how to store and update on-chain state using a simple counter.

The contract allows users to initialize, increment, and retrieve a stored value securely on the blockchain.

---

## ⚙️ What it does

* Initializes a counter value on-chain
* Allows users to increment the counter
* Retrieves the current counter value

This serves as a foundational example for understanding Soroban smart contract development.

---

## ✨ Features

* ✅ Persistent on-chain storage
* ✅ Simple and efficient logic
* ✅ Easy to extend (e.g., add access control, limits, etc.)
* ✅ Beginner-friendly structure
* ✅ Built with Rust and Soroban SDK

---

## 🔗 Deployed Smart Contract Link

**Contract Address:**
`CDJN7PP26H773IZ45SZP3SEF3G3CH4RZODLM3Y4MWUQISEMIRT73CLBT`

You can interact with this contract using the Soroban CLI or integrate it into your frontend application.

---

## 🛠️ How to Run Locally

### 1. Install Stellar CLI

```bash
cargo install soroban-cli
```

### 2. Build the contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 3. Deploy

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/counter_contract.wasm \
  --source <YOUR_IDENTITY> \
  --network testnet
```

---

## 📈 Future Improvements

* Add user-specific counters
* Add access control (admin roles)
* Add decrement/reset functions
* Integrate frontend (React / Next.js)

---

## 🧑‍💻 Author

Your Name
<img width="1440" height="774" alt="image" src="https://github.com/user-attachments/assets/6a023340-1f07-49c8-93b2-90e746ff6bda" />

