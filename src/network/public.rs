use k256::{
    SecretKey,
    elliptic_curve::rand_core::{RngCore, OsRng},
};

// A `k256` key pair.
pub struct KeyPair {
    public: Vec<u8>,
    secret: Vec<u8>,
}

impl KeyPair {
    pub fn generate() -> Option<KeyPair> {
        // let rng    = OsRng::new().ok()?;
        let secret = SecretKey::random(OsRng);
        let public = secret.public_key();

        println!("{:#?}", public.to_projective());
        panic!();

        // Some(KeyPair {
        //     public: public,
        //     secret: secret.to_bytes().as_slice().to_vec(),
        // })
    }
}
