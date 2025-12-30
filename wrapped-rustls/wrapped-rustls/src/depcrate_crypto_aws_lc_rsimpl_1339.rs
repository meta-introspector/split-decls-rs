// Generated macro for impl_1339 (impl)
macro_rules! Depcrate_crypto_aws_lc_rsimpl_1339 {
() => {
// Module: crate::crypto::aws_lc_rs
// Provides: {"impl_1339"}
// Dependencies: {}
impl TicketerFactory for AwsLcRs { # [doc = " Make the recommended `Ticketer`."] # [doc = ""] # [doc = " This produces tickets:"] # [doc = ""] # [doc = " - where each lasts for at least 6 hours,"] # [doc = " - with randomly generated keys, and"] # [doc = " - where keys are rotated every 6 hours."] # [doc = ""] # [doc = " The `Ticketer` uses the [RFC 5077 §4] \"Recommended Ticket Construction\","] # [doc = " using AES 256 for encryption and HMAC-SHA256 for ciphertext authentication."] # [doc = ""] # [doc = " [RFC 5077 §4]: https://www.rfc-editor.org/rfc/rfc5077#section-4"] fn ticketer (& self) -> Result < Arc < dyn TicketProducer > , Error > { # [cfg (feature = "std")] { Ok (Arc :: new (TicketRotator :: new (TicketRotator :: SIX_HOURS , Rfc5077Ticketer :: new ,) ?)) } # [cfg (not (feature = "std"))] { Err (Error :: General ("AwsLcRs::ticketer() relies on std-only RwLock via TicketRotator" . into () ,)) } } fn fips (& self) -> bool { fips () } }
};
}
