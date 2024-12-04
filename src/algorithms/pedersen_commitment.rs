// Set prime number (p) and generator (g)

// Generate 'h' with a random number 'r' (h = g^r mod p)
// TODO: Generate a random number r (and save it to this.r)
// TODO: Calculate h using g, r and p (and save it to this.h)

// Generate the commitment (g^s * h^r mod p)
// TODO: Convert s to BigInt (and save it to this.s)
// TODO: Calculate and return the commitment using g, s, h, r and p

// Reveal the secret number and random number (s, r)
// TODO: Return the secret and random number

// Verify the commitment (g^s * h^r mod p)
// TODO: Verify the commitment by recalculating it and comparing with C

// Test the PedersenCommitment

// Party A: Generate a commitment

// Party A: Reveal the secret and random number

// Party B: Verify the commitment

use num_bigint::RandBigInt;
use rand::{thread_rng, Rng};

pub fn pedersen_commitment() {
    let mut rng = thread_rng();
    // let p = rng.gen_biguint(23);
    // let g = rng.gen_biguint(4);
    let p: u128 = 23;
    let g: u128 = 4;

    let r: u32 = rng.gen_range(2..23);

    println!("Random large number is {p}");
    println!("Random large number is {g}");
}
