use rand_core::{RngCore, OsRng};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};

use crate::network::key::KeyPair;

// TODO: sign message
/// Using a secret key pair and the recipient's public key,
/// this function encrypts a message for them,
/// returning the nonce and the encrypted message.
/// Uses ChaCha20Poly1305 - HKDF + ECC are used to generate the shared secret.
pub fn encrypt_message(
    secret_key_pair: &KeyPair,
    other_public_bytes: &[u8],
    message: Vec<u8>,
) -> Option<([u8; 12], Vec<u8>)> {
    let shared_secret = secret_key_pair.shared_secret(other_public_bytes)?;
    let key = Key::from_slice(&shared_secret);

    let mut proto_nonce = [0u8; 12];
    OsRng.fill_bytes(&mut proto_nonce);
    let nonce = Nonce::from_slice(&proto_nonce);

    println!("secret: {:?}", super::key::hex(&shared_secret));
    println!("nonce: {:?}", super::key::hex(&proto_nonce));

    let cipher = ChaCha20Poly1305::new(key);
    let encrypted = cipher.encrypt(&nonce, message.as_ref()).ok()?;

    let plaintext = cipher.decrypt(&nonce, encrypted.as_ref()).unwrap();
    println!("message: {:?}", message);
    println!("done");
    return Some((proto_nonce, encrypted));
}

/// Using a secret key pair, the recipient's public key, and the message nonce,
/// this function decrypts a message from them,
/// returning the decrypted message.
/// To be used with `encrypt_message`.
pub fn decrypt_message(
    secret_key_pair: &KeyPair,
    other_public_bytes: &[u8],
    proto_nonce: [u8; 12],
    encrypted: Vec<u8>,
) -> Option<Vec<u8>> {
    let shared_secret = secret_key_pair.shared_secret(other_public_bytes)?;
    let key = Key::from_slice(&shared_secret);
    let nonce = Nonce::from_slice(&proto_nonce);

    println!("secret: {:?}", super::key::hex(&shared_secret));
    println!("nonce: {:?}", super::key::hex(&proto_nonce));

    let cipher = ChaCha20Poly1305::new(key);

    let message = cipher.decrypt(&nonce, encrypted.as_ref());
    println!("message: {:?}", message);
    println!("done");
    return Some(message.ok()?);
}
