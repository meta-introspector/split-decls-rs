// Generated macro for sign_message (function)
macro_rules! Depcratesign_message {
() => {
// Module: crate
// Provides: {"sign_message"}
// Dependencies: {}
# [doc = " Signs a message from the given private key bytes"] pub fn sign_message (priv_key_bytes : & [u8 ; SECP256K1_PRIVATE_KEY_SIZE] , message : & [u8] ,) -> Result < ([u8 ; SIGNATURE_SERIALIZED_SIZE] , u8) , Error > { let priv_key = k256 :: ecdsa :: SigningKey :: from_slice (priv_key_bytes) . map_err (| e | Error :: from_source (format ! ("{e}"))) ? ; let mut hasher = sha3 :: Keccak256 :: new () ; hasher . update (message) ; let message_hash = hasher . finalize () ; let mut message_hash_arr = [0u8 ; 32] ; message_hash_arr . copy_from_slice (message_hash . as_slice ()) ; let (signature , recovery_id) = priv_key . sign_prehash_recoverable (& message_hash_arr) . map_err (| e | Error :: from_source (format ! ("{e}"))) ? ; Ok ((signature . to_bytes () . into () , recovery_id . to_byte ())) }
};
}
