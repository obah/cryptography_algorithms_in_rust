mod algorithms;

use algorithms::{diffie_hellman, elgamal, rsa_encryption_and_signature, sha256_and_poseidon_hashing, merkle_tree};

fn main() {
    diffie_hellman::diffie_hellman();
    elgamal::elgamal();
    rsa_encryption_and_signature::rsa_encryt_decrypt();
    rsa_encryption_and_signature::digital_signer();
    sha256_and_poseidon_hashing::sha256_hasher();
    sha256_and_poseidon_hashing::poseidon_hasher();
    merkle_tree::merkle();
    // pedersen_commitment::pedersen_commitment();
}
