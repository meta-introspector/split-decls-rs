// Generated macro for impl_1203 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_kximpl_1203 {
() => {
// Module: crate::crypto::aws_lc_rs::kx
// Provides: {"impl_1203"}
// Dependencies: {}
impl SupportedKxGroup for KxGroup { fn start (& self) -> Result < StartedKeyExchange , Error > { let rng = SystemRandom :: new () ; let priv_key = agreement :: EphemeralPrivateKey :: generate (self . agreement_algorithm , & rng) . map_err (| _ | GetRandomFailed) ? ; let pub_key = priv_key . compute_public_key () . map_err (| _ | GetRandomFailed) ? ; Ok (StartedKeyExchange :: Single (Box :: new (KeyExchange { name : self . name , agreement_algorithm : self . agreement_algorithm , priv_key , pub_key , pub_key_validator : self . pub_key_validator , }))) } fn name (& self) -> NamedGroup { self . name } fn fips (& self) -> bool { self . fips_allowed && super :: fips () } }
};
}
