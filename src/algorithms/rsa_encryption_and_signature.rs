use rand::{rngs::ThreadRng, thread_rng};
use rsa::pkcs1v15::SigningKey;
use rsa::sha2::Sha256;
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

fn get_keys() -> (ThreadRng, RsaPrivateKey, RsaPublicKey) {
    let mut rng = thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    (rng, priv_key, pub_key)
}

// Asymmetric encryption
pub fn rsa_encryt_decrypt() {
    let (mut rng, priv_key, pub_key) = get_keys();

    let plaintext = b"This is a secret message from Obaloluwa.";
    println!("The original text is {:?}", plaintext);

    //encrypt
    let enc_text = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &plaintext[..])
        .expect("Failed to encrypt");
    assert_ne!(&plaintext[..], &enc_text[..]);
    println!("The encrypted text is {:?}", enc_text);

    //decrypt
    let dec_text = priv_key
        .decrypt(Pkcs1v15Encrypt, &enc_text)
        .expect("failed to decrypt");
    assert_eq!(&plaintext[..], &dec_text[..]);
    println!("the decrypted text is {:?}", dec_text);
}

//digital signature
pub fn digital_signer() {
    println!("_____starting the signer________");
    let (mut rng, priv_key, _) = get_keys();

    let message = b"This is a secret message from Obaloluwa.";

    let signing_key = SigningKey::<Sha256>::new(priv_key);
    let verifying_key = signing_key.verifying_key();

    println!("______signing________");
    //sign the message
    let signature = signing_key.sign_with_rng(&mut rng, message);
    assert_ne!(signature.to_bytes().as_ref(), message.as_slice());

    println!("______verifying_____)");

    //verify the message
    verifying_key
        .verify(message, &signature)
        .expect("Failed to verify");

    println!("______done_______");
}
