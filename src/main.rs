// backend
pub mod datastore;
pub mod network;

// frontends
pub mod ui;
pub mod cli;

pub fn main() {
    println!("\nSmall ecc key-pair test");
    println!("-----------------------");

    let my_key   = network::key::KeyPair::generate();
    let your_key = network::key::KeyPair::generate();

    println!("my public key:   {}", network::key::hex(my_key.public_bytes()));
    println!("your public key: {}", network::key::hex(your_key.public_bytes()));
    println!("-----------------------");

    let my_side_shared   = my_key.shared_secret(&your_key.public_bytes());
    let your_side_shared = your_key.shared_secret(&my_key.public_bytes());

    println!("my side shared secret:   {}", network::key::hex(my_side_shared.unwrap().to_vec()));
    println!("your side shared secret: {}", network::key::hex(your_side_shared.unwrap().to_vec()));
    println!("-----------------------");

    println!("test passed: {}\n", my_side_shared == your_side_shared);

    // ui::run();
}
