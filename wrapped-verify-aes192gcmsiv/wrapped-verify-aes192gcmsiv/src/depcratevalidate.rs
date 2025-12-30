// Generated macro for validate (function)
macro_rules! Depcratevalidate {
() => {
// Module: crate
// Provides: {"validate"}
// Dependencies: {}
fn validate (v : & VectorArgs) { let key_bytes = hex :: decode (& v . key) . unwrap () ; let nonce_bytes = hex :: decode (& v . nonce) . unwrap () ; let aad_bytes = hex :: decode (& v . aad) . unwrap () ; let plaintext_bytes = hex :: decode (& v . plaintext) . unwrap () ; let expected_ciphertext_bytes = hex :: decode (& v . ciphertext) . unwrap () ; let expected_tag_bytes = hex :: decode (& v . tag) . unwrap () ; let key_array : [u8 ; 24] = key_bytes . try_into () . unwrap () ; let cipher = Aes192GcmSiv :: new (& GenericArray :: from (key_array)) ; let payload = Payload { msg : plaintext_bytes . as_slice () , aad : aad_bytes . as_slice () , } ; let encrypted_bytes = cipher . encrypt (Nonce :: from_slice (nonce_bytes . as_slice ()) , payload) . unwrap () ; let (ciphertext_bytes , tag_bytes) = encrypted_bytes . split_at (plaintext_bytes . len ()) ; assert_eq ! (ciphertext_bytes , expected_ciphertext_bytes) ; assert_eq ! (tag_bytes , expected_tag_bytes) ; }
};
}
