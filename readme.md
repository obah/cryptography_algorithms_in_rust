# Cryptography Concepts Implementation

This project demonstrates various fundamental cryptographic concepts and techniques implemented in Rust. It serves as a practical exploration of different cryptographic primitives and techniques.

## Modules

### 1. Hash Functions (`sha256_and_poseidon_hashing.rs`)
- Implementation of two different hash functions:
  - **SHA-256**: A widely used cryptographic hash function from the SHA-2 family
  - **Poseidon**: A zero-knowledge proof friendly hash function optimized for arithmetic circuits

### 2. RSA Cryptography (`rsa_encryption_and_signature.rs`)
- Demonstrates two core RSA functionalities:
  - **Asymmetric Encryption**: Implementation of RSA encryption/decryption
  - **Digital Signatures**: RSA-based digital signature creation and verification
- Uses 2048-bit keys for security

### 3. ElGamal Encryption (`elgamal.rs`)
- Implementation of the ElGamal public-key cryptosystem
- Includes:
  - Key generation
  - Message encryption
  - Message decryption
- Based on the difficulty of the discrete logarithm problem

### 4. Merkle Tree (`merkle_tree.rs`)
- Implementation of a Merkle Tree data structure
- Features:
  - Tree construction from leaf values
  - Merkle proof generation
  - Proof verification
- Uses SHA-256 as the underlying hash function

### 5. Diffie-Hellman Key Exchange (`diffie_hellman.rs`)
- Implementation of the Diffie-Hellman key exchange protocol
- Demonstrates how two parties can establish a shared secret over an insecure channel
- Includes public and private key generation and shared secret computation

### 6. Pedersen Commitment (`pedersen_commitment.rs`)
- (Currently commented out in main.rs as it is uncompleted)
- Implementation of the Pedersen commitment scheme
- A cryptographic primitive that allows committing to a value while keeping it hidden

## Usage

The `main.rs` file demonstrates the usage of each cryptographic implementation. To run the project:

```bash
    cargo run
```

## Dependencies

- `dusk-bls12-381`: For BLS12-381 curve operations
- `dusk-poseidon`: For Poseidon hash implementation
- `rsa`: For RSA cryptography
- `rand`: For secure random number generation
- `rs-merkle`: For Merkle tree implementation

## Purpose

This project serves as an educational resource for understanding various cryptographic concepts through practical implementation. It's important to note that while these implementations demonstrate the concepts, they may not be suitable for production use without additional security considerations and auditing.

## Security Note

These implementations are for learning purposes and may not include all the necessary security measures required for production systems. For real-world applications, it's recommended to use well-audited cryptographic libraries.
