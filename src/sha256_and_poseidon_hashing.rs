use dusk_bls12_381::BlsScalar;
use dusk_poseidon::{Domain, Hash};
use rsa::sha2::{Digest, Sha256};

// SHA-256
pub fn sha256_hasher() {
    let data = b"This is some data X.";

    let mut data_hash = Sha256::new();

    data_hash.update(data);

    let result = data_hash.finalize();

    println!("The original message is {:?}", data);
    println!("The hashed message is {:?}", result);
}

// Poseidon
pub fn poseidon_hasher() {
    let inputs = BlsScalar::from_raw([1, 2, 3, 4]);

    let hash = Hash::digest(Domain::Other, &[inputs]);

    println!("Original input is: {:?}", inputs);
    println!("Poseidon has of input is {:?}", hash);
}
