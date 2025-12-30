// Generated macro for macros (module)
macro_rules! Depcratemacros {
() => {
// Module: crate
// Provides: {"macros"}
// Dependencies: {}
pub mod macros { # ! [doc = " Macros that bring a provider into the current scope."] # ! [doc = ""] # ! [doc = " The selected provider module is bound as `provider`; you can rely on this"] # ! [doc = " having the union of the public items common to the `rustls::crypto::ring`"] # ! [doc = " and `rustls::crypto::aws_lc_rs` modules."] # [macro_export] macro_rules ! provider_ring { () => { # [allow (unused_imports)] use rustls :: crypto :: ring as provider ; # [allow (dead_code)] const fn provider_is_aws_lc_rs () -> bool { false } # [allow (dead_code)] const fn provider_is_ring () -> bool { true } # [allow (dead_code)] const fn provider_is_fips () -> bool { false } # [allow (dead_code)] const ALL_VERSIONS : [rustls :: crypto :: CryptoProvider ; 2] = [provider :: DEFAULT_TLS12_PROVIDER , provider :: DEFAULT_TLS13_PROVIDER ,] ; } ; } # [macro_export] macro_rules ! provider_aws_lc_rs { () => { # [allow (unused_imports)] use rustls :: crypto :: aws_lc_rs as provider ; # [allow (dead_code)] const fn provider_is_aws_lc_rs () -> bool { true } # [allow (dead_code)] const fn provider_is_ring () -> bool { false } # [allow (dead_code)] const fn provider_is_fips () -> bool { cfg ! (feature = "fips") } # [allow (dead_code)] const ALL_VERSIONS : [rustls :: crypto :: CryptoProvider ; 2] = [provider :: DEFAULT_TLS12_PROVIDER , provider :: DEFAULT_TLS13_PROVIDER ,] ; } ; } }
};
}
