use std::num::NonZeroU32;

use ring::pbkdf2::{derive, PBKDF2_HMAC_SHA512};
use ring::rand::{SecureRandom, SystemRandom};

pub fn get_rand_16() -> [u8; 16] {
    let random = SystemRandom::new();
    let mut buf = [0; 16];
    random.fill(&mut buf).unwrap();
    buf
}

pub fn derive_stored_password(plaintext: &Vec<u8>, salt: &[u8; 16]) -> [u8; 64] {
    let mut buf = [0; 64];
    let iters = NonZeroU32::new(100_000).unwrap();
    derive(PBKDF2_HMAC_SHA512, iters, &salt[..], plaintext, &mut buf);
    buf
}
