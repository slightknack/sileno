use k256::{
    EncodedPoint,
    PublicKey,
    ecdh::{
        EphemeralSecret,
        SharedSecret,
    }
};
use rand_core::OsRng;
use hkdf::Hkdf;
use sha2::Sha256;

// Converts a vector of bytes to a string hex representation.
pub fn hex(bytes: Vec<u8>) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// A k256 elliptic curve key pair (assymetric crypto, ephemeral).
/// Basically just wraps some primitives provided by the crate `k256` of RustCrypto.
/// We only store the secret key, because the public key can be derived.
/// To be used with Diffie Hellman:
/// to send some data, we use the other person's public key to generate a shared secret,
/// then encrypt it with chacha20poly1305 (a fast symmetric cryptosystem),
/// and send that over the wire.
pub struct KeyPair {
    secret: EphemeralSecret,
}

impl KeyPair {
    /// Generates a new `KeyPair` using the OS-level random number generator.
    pub fn generate() -> KeyPair {
        let secret = EphemeralSecret::random(&mut OsRng);
        let public = secret.public_key();

        KeyPair { secret }
    }

    /// Returns the public key, as a vector of bytes.
    /// This can be used along with a different secret key to generate a shared secret key.
    pub fn public_bytes(&self) -> Vec<u8> {
        EncodedPoint::from(self.secret.public_key()).as_bytes().to_vec()
    }

    /// Uses the HKDF to expand the non-unformly distributed shared secret
    /// into a uniformly shared secret.
    /// This returns 32 byte (256 bit) key, enough for ChaCha20Poly1305.
    fn expand_secret(secret: SharedSecret) -> Option<[u8; 32]> {
        let hk = Hkdf::<Sha256>::new(None, &secret.as_bytes());
        let mut okm = [0u8; 32];
        // TODO: supply context?
        hk.expand(&[], &mut okm).ok();
        return Some(okm);
    }

    /// Returns a 64-byte uniformly-distributed shared secret key,
    /// given the byte-representation of a different public key.
    pub fn shared_secret(&self, other_public_bytes: &[u8]) -> Option<[u8; 32]> {
        let other_public = PublicKey::from_sec1_bytes(other_public_bytes).ok()?;
        let shared = self.secret.diffie_hellman(&other_public);
        return KeyPair::expand_secret(shared);
    }
}
