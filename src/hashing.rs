

pub fn get_hash_and_salt(password: &str) {
    // const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    // let n_iter = NonZeroU32::new(100_000).unwrap();
    // let rng = rand::SystemRandom::new();

    // let mut salt = [0u8; CREDENTIAL_LEN];
    // rng.fill(&mut salt)?;

    // let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    // pbkdf2::derive(
    //     pbkdf2::PBKDF2_HMAC_SHA512,
    //     n_iter,
    //     &salt,
    //     password.as_bytes(),
    //     &mut pbkdf2_hash,
    // );
    // println!("Salt: {}", HEXUPPER.encode(&salt));
    // println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));


}

pub fn check_hash(password: &str, hash: &str, salt: &str) {
    // let should_succeed = pbkdf2::verify(
    //     pbkdf2::PBKDF2_HMAC_SHA512,
    //     n_iter,
    //     &salt,
    //     password.as_bytes(),
    //     &pbkdf2_hash,
    // );
}