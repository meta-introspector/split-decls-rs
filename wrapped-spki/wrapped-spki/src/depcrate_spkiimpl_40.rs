// Generated macro for impl_40 (impl)
macro_rules! Depcrate_spkiimpl_40 {
() => {
// Module: crate::spki
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a , Params , Key > SubjectPublicKeyInfo < Params , Key > where Params : Choice < 'a , Error = der :: Error > + Encode , Key : Decode < 'a , Error = der :: Error > + Encode + FixedTag , { # [doc = " Calculate the SHA-256 fingerprint of this [`SubjectPublicKeyInfo`] and"] # [doc = " encode it as a Base64 string."] # [doc = ""] # [doc = " See [RFC7469 § 2.1.1] for more information."] # [doc = ""] # [doc = " [RFC7469 § 2.1.1]: https://datatracker.ietf.org/doc/html/rfc7469#section-2.1.1"] # [cfg (all (feature = "fingerprint" , feature = "alloc" , feature = "base64"))] pub fn fingerprint_base64 (& self) -> Result < alloc :: string :: String > { use base64ct :: { Base64 , Encoding } ; Ok (Base64 :: encode_string (& self . fingerprint_bytes () ?)) } # [doc = " Calculate the SHA-256 fingerprint of this [`SubjectPublicKeyInfo`] as"] # [doc = " a raw byte array."] # [doc = ""] # [doc = " See [RFC7469 § 2.1.1] for more information."] # [doc = ""] # [doc = " [RFC7469 § 2.1.1]: https://datatracker.ietf.org/doc/html/rfc7469#section-2.1.1"] # [cfg (feature = "fingerprint")] pub fn fingerprint_bytes (& self) -> Result < FingerprintBytes > { let mut hash = Sha256 :: new () ; self . encode (& mut DigestWriter (& mut hash)) ? ; Ok (hash . finalize () . into ()) } }
};
}
