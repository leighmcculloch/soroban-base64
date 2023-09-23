#![no_std]

use soroban_sdk::{contract, contractimpl, BytesN, Env, String};

mod base64_url;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Return base64url encoded hash.
    pub fn encode(e: &Env, hash: BytesN<32>) -> String {
        let mut encoded = *b"___________________________________________";
        base64_url::encode(&mut encoded, &hash.to_array());
        String::from_bytes(e, &encoded)
    }
}
