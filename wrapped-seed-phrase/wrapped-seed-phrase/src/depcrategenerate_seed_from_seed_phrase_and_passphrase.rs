// Generated macro for generate_seed_from_seed_phrase_and_passphrase (function)
macro_rules! Depcrategenerate_seed_from_seed_phrase_and_passphrase {
() => {
// Module: crate
// Provides: {"generate_seed_from_seed_phrase_and_passphrase"}
// Dependencies: {}
pub fn generate_seed_from_seed_phrase_and_passphrase (seed_phrase : & str , passphrase : & str ,) -> Vec < u8 > { const PBKDF2_ROUNDS : u32 = 2048 ; const PBKDF2_BYTES : usize = 64 ; let salt = format ! ("mnemonic{passphrase}") ; let mut seed = vec ! [0u8 ; PBKDF2_BYTES] ; pbkdf2 :: pbkdf2 :: < Hmac < sha2 :: Sha512 > > (seed_phrase . as_bytes () , salt . as_bytes () , PBKDF2_ROUNDS , & mut seed ,) ; seed }
};
}
