// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
# [cfg (any (test , feature = "verify"))] impl Signature { pub (self) fn verify_verbose (& self , pubkey_bytes : & [u8] , message_bytes : & [u8] ,) -> Result < () , ed25519_dalek :: SignatureError > { let publickey = ed25519_dalek :: VerifyingKey :: try_from (pubkey_bytes) ? ; let signature = self . 0 . as_slice () . try_into () ? ; publickey . verify_strict (message_bytes , & signature) } pub fn verify (& self , pubkey_bytes : & [u8] , message_bytes : & [u8]) -> bool { self . verify_verbose (pubkey_bytes , message_bytes) . is_ok () } }
};
}
