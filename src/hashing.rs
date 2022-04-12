
use bcrypt;

pub fn get_hash_and_salt(password: &str) -> Result<(String, String), bcrypt::BcryptError>{
    let hashing_result = bcrypt::hash_with_result(password, bcrypt::DEFAULT_COST)?;
    
    Ok((hashing_result.format_for_version(bcrypt::Version::TwoB), hashing_result.get_salt()))
}

pub fn check_hash(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError>{
    let verification_result = bcrypt::verify(password, hash);

    verification_result
}