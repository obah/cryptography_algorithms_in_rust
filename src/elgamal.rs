use rand::{thread_rng, Rng};

//find the gcd in the field
fn calc_gcd(a: u128, b: u128) -> u128 {
    if a < b {
        calc_gcd(b, a)
    } else if a % b == 0 {
        b
    } else {
        calc_gcd(b, a % b)
    }
}

//generate large number
fn gen_key(q: u128) -> u128 {
    let mut rng = thread_rng();

    let mut key = rng.gen_range(10_u128.pow(5)..q);

    while calc_gcd(q, key) != 1 {
        key = rng.gen_range(10_u128.pow(5)..q);
    }

    key
}

//modular exponentiation
fn calc_power(a: u128, b: u128, c: u128) -> u128 {
    let mut x = 1;
    let mut y = a;
    let mut z = b;

    while z > 0 {
        if z % 2 != 0 {
            x = (x * y) % c;
        }

        y = (y * y) % c;

        z = z / 2
    }

    x % c
}

// asymmetric encryption
fn encrypt(msg: &str, q: u128, h: u128, g: u128) -> (Vec<u128>, u128) {
    let mut msg_chars: Vec<char> = vec![];

    let k = gen_key(q); // Private key for sender
    let s = calc_power(h, k, q);
    let p = calc_power(g, k, q);

    for ch in msg.chars() {
        msg_chars.push(ch);
    }

    let mut en_msg: Vec<u128> = vec![0; msg_chars.len()];

    println!("g^k used : {p}");
    println!("g^ak used : {s}");

    for i in 0..msg_chars.len() {
        let char_num: u128 = (msg_chars[i] as u32).into();
        en_msg[i] = s * char_num;
    }

    return (en_msg, p);
}

fn decrypt(en_msg: Vec<u128>, p: u128, key: u128, q: u128) -> Vec<char> {
    let mut dr_msg = vec![];

    let h = calc_power(p, key, q);

    for i in 0..en_msg.len() {
        let ch: u8 = (en_msg[i] / h).try_into().unwrap();
        dr_msg.push(ch as char);
    }

    dr_msg
}

pub fn elgamal() {
    let mut rng = thread_rng();

    let msg = "encryption";
    println!("Original message is `{msg}`");

    let q: u128 = rng.gen_range(10_u128.pow(5)..10_u128.pow(10));
    let g: u128 = rng.gen_range(2..q);

    let key: u128 = gen_key(q); //private key for receiver
    println!("key is {key}");

    let h = calc_power(g, key, q);

    println!("g used: {g}");
    println!("g^a used: {h}");

    let (en_msg, p) = encrypt(msg, q, h, g);
    let dr_msg = decrypt(en_msg, p, key, q);

    let dmsg: String = dr_msg.iter().collect();

    println!("Decrypted message is `{dmsg}`");
}
