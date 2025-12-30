// Generated macro for Cipher (struct)
macro_rules! Depcrate_pkeCipher {
() => {
// Module: crate::pke
// Provides: {"Cipher"}
// Dependencies: {}
# [doc = " Represents a cipher structure containing encryption-related data (asn.1 format)."] # [doc = ""] # [doc = " The `Cipher` structure includes the coordinates of the elliptic curve point (`x`, `y`),"] # [doc = " the digest of the message, and the encrypted cipher text."] pub struct Cipher < 'a > { x : U256 , y : U256 , digest : & 'a [u8] , cipher : & 'a [u8] , }
};
}
