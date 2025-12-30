// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl Secp256k1Pubkey { pub fn new (pubkey_vec : & [u8]) -> Self { Self (< [u8 ; SECP256K1_PUBLIC_KEY_LENGTH] > :: try_from (< & [u8] > :: clone (& pubkey_vec)) . expect ("Slice must be the same length as a Pubkey") ,) } pub fn to_bytes (self) -> [u8 ; 64] { self . 0 } }
};
}
