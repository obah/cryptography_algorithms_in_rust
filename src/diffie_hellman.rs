// //calculates the key for 2 parties alice and bob using the diffie-hellman key exchange algorithm

fn calc_power(p_gen: u32, priv_key: u32, p: u32) -> u32 {
    if priv_key == 1 {
        p_gen
    } else {
        p_gen.pow(priv_key) % p
    }
}

pub fn diffie_hellman() {
    let p = 23u32;
    let g = 9u32; //generator of p
    let a = 4u32;
    let b = 3u32;
    let mut x = 0u32;
    let mut y = 0u32;
    let mut ka = 0u32;
    let mut kb: u32 = 0u32;

    println!("The value of P and our modulus is {p}");
    println!("The value of G and our generator is {g}");
    println!("Alice's private key is {a}");
    println!("Bob's private key is {b}");

    //generate public keys
    x = calc_power(g, a, p);
    y = calc_power(g, b, p);

    println!("public key for alice is {x}");
    println!("public key for bob is {y}");

    //generate secret keys with exchanged public keys
    ka = calc_power(y, a, p);
    kb = calc_power(x, b, p);

    println!("Alice's secret  key is {ka}");
    println!("Bob's secret  key is {kb}");
}
