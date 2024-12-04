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

fn gen_random_key(p: u128) -> u128 {
    let mut rng = thread_rng();

    let mut key = rng.gen_range(10_u128.pow(5)..p);

    while calc_gcd(p, key) != 1 {
        key = rng.gen_range(10_u128.pow(5)..p);
    }

    key
}

fn mod_exponentiation(mut a: u128, mut b: u128, c: u128) -> u128 {
    let mut x = 1;

    while b > 0 {
        if b % 2 != 0 {
            x = (x * a) % c;
        }

        a = (a * a) % c;

        b = b / 2
    }

    x % c
}

fn encrypt(msg: &str, q: u128, h: u128, g: u128) -> (Vec<u128>, u128) {
    let mut msg_chars: Vec<char> = vec![];

    let k = gen_random_key(q); // Private key for sender
    let g_k = mod_exponentiation(g, k, q);
    let g_a_k = mod_exponentiation(h, k, q);

    for ch in msg.chars() {
        msg_chars.push(ch);
    }

    let mut en_msg: Vec<u128> = vec![0; msg_chars.len()];

    println!("g^k used : {g_k}");
    println!("g^ak used : {g_a_k}");

    for i in 0..msg_chars.len() {
        let char_num: u128 = (msg_chars[i] as u32).into();
        en_msg[i] = g_a_k * char_num;
    }

    return (en_msg, g_k);
}

fn decrypt(en_msg: &Vec<u128>, p: u128, key: u128, q: u128) -> Vec<char> {
    let mut dr_msg = vec![];

    let h = mod_exponentiation(p, key, q);

    for i in 0..en_msg.len() {
        let ch: u8 = (en_msg[i] / h).try_into().unwrap();
        dr_msg.push(ch as char);
    }

    dr_msg
}

pub fn elgamal() {
    let mut rng = thread_rng(); //initialise the random generator

    let p: u128 = rng.gen_range(10_u128.pow(5)..10_u128.pow(10)); //random number representing our large prime
    let g: u128 = rng.gen_range(2..p); //random number representing the generator

    let msg = "super secret";

    let key: u128 = gen_random_key(p); //private key for receiver
    println!("key is {key}");

    let g_a = mod_exponentiation(g, key, p);

    let (en_msg, g_k) = encrypt(msg, p, g_a, g);
    let dr_msg = decrypt(&en_msg, g_k, key, p);

    let dc_msg: String = dr_msg.iter().collect();

    println!("Original message is `{msg}`");
    println!("Encrypted message is `{:?}`", en_msg);
    println!("Decrypted message is `{dc_msg}`");
}
