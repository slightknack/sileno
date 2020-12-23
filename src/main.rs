// backend
pub mod datastore;
pub mod network;

// frontends
pub mod ui;
pub mod cli;

pub fn main() {
    network::public::KeyPair::generate();
    // ui::run();
}
