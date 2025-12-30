// Generated macro for kdf (function)
macro_rules! Depcrate_pkekdf {
() => {
// Module: crate::pke
// Provides: {"kdf"}
// Dependencies: {}
# [doc = " Performs key derivation using a hash function and elliptic curve point."] fn kdf (hasher : & mut dyn DynDigest , kpb : AffinePoint , c2 : & mut [u8]) -> Result < () > { let klen = c2 . len () ; let mut ct : i32 = 0x00000001 ; let mut offset = 0 ; let digest_size = hasher . output_size () ; let mut ha = vec ! [0u8 ; digest_size] ; let encode_point = kpb . to_encoded_point (false) ; while offset < klen { hasher . update (encode_point . x () . ok_or (elliptic_curve :: Error) ?) ; hasher . update (encode_point . y () . ok_or (elliptic_curve :: Error) ?) ; hasher . update (& ct . to_be_bytes ()) ; hasher . finalize_into_reset (& mut ha) . map_err (| _e | elliptic_curve :: Error) ? ; let xor_len = min (digest_size , klen - offset) ; xor (c2 , & ha , offset , xor_len) ; offset += xor_len ; ct += 1 ; } Ok (()) }
};
}
