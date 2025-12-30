// Generated macro for impl_92 (impl)
macro_rules! Depcrate_extimpl_92 {
() => {
// Module: crate::ext
// Provides: {"impl_92"}
// Dependencies: {}
impl Nonce { # [doc = " Creates a Nonce object given the bytes"] pub fn new (bytes : impl Into < Box < [u8] > >) -> Result < Self , der :: Error > { Ok (Self (OctetString :: new (bytes) ?)) } # [doc = " Creates a Nonce object given a random generator and a length."] # [doc = ""] # [doc = " A proposed but not (yet) accepted RFC [RFC 8954] wants to limit nonces. RFC 6960 has no"] # [doc = " mention of a minimum or maximum length."] # [doc = ""] # [doc = " ```text"] # [doc = " Nonce ::= OCTET STRING(SIZE(1..32))"] # [doc = " ```"] # [cfg (feature = "rand")] pub fn generate < R > (rng : & mut R , length : usize) -> Result < Self , der :: Error > where R : CryptoRng + ? Sized , { let mut bytes = alloc :: vec ! [0 ; length] ; rng . fill_bytes (& mut bytes) ; Self :: new (bytes) } }
};
}
