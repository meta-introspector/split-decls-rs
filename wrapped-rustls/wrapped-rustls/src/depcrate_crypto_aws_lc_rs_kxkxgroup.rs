// Generated macro for KxGroup (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_kxKxGroup {
() => {
// Module: crate::crypto::aws_lc_rs::kx
// Provides: {"KxGroup"}
// Dependencies: {}
# [doc = " A key-exchange group supported by *ring*."] struct KxGroup { # [doc = " The IANA \"TLS Supported Groups\" name of the group"] name : NamedGroup , # [doc = " The corresponding ring agreement::Algorithm"] agreement_algorithm : & 'static agreement :: Algorithm , # [doc = " Whether the algorithm is allowed by FIPS"] # [doc = ""] # [doc = " `SupportedKxGroup::fips()` is true if and only if the algorithm is allowed,"] # [doc = " _and_ the implementation is FIPS-validated."] fips_allowed : bool , # [doc = " aws-lc-rs 1.9 and later accepts more formats of public keys than"] # [doc = " just uncompressed."] # [doc = ""] # [doc = " That is not compatible with TLS:"] # [doc = " - TLS1.3 outlaws other encodings,"] # [doc = " - TLS1.2 negotiates other encodings (we only offer uncompressed), and"] # [doc = "   defaults to uncompressed if negotiation is not done."] # [doc = ""] # [doc = " This function should return `true` if the basic shape of its argument"] # [doc = " is consistent with an uncompressed point encoding.  It does not need"] # [doc = " to verify that the point is on the curve (if the curve requires that"] # [doc = " for security); aws-lc-rs/ring must do that."] pub_key_validator : fn (& [u8]) -> bool , }
};
}
