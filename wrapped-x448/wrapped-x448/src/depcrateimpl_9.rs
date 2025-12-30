// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl PublicKey { # [doc = " Converts a bytes slice into a Public key"] # [doc = " Returns None if:"] # [doc = " -  The length of the slice is not 56"] # [doc = " -  The point is a low order point"] pub fn from_bytes (bytes : & [u8]) -> Option < PublicKey > { let public_key = PublicKey :: from_bytes_unchecked (bytes) ? ; if public_key . 0 . is_low_order () { return None ; } Some (public_key) } # [doc = " Converts a bytes slice into a Public key"] # [doc = " Returns None if:"] # [doc = " -  The length of the slice is not 56"] pub fn from_bytes_unchecked (bytes : & [u8]) -> Option < PublicKey > { if bytes . len () != 56 { return None ; } let arr = slice_to_array (bytes) ; let point = MontgomeryPoint (arr) ; Some (PublicKey (point)) } # [doc = " Converts a public key into a byte slice"] pub fn as_bytes (& self) -> & [u8 ; 56] { self . 0 . as_bytes () } }
};
}
