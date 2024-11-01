mod diffie_hellman;
mod elgamal;
mod rsa_encryption_and_signature;
mod sha256_and_poseidon_hashing;

fn main() {
    // diffie_hellman::diffie_hellman();
    // elgamal::elgamal();
    // rsa_encryption_and_signature::rsa_encryt_decrypt();
    rsa_encryption_and_signature::digital_signer();
}
