// Generated macro for default_fips_provider (function)
macro_rules! Depcrate_cryptodefault_fips_provider {
() => {
// Module: crate::crypto
// Provides: {"default_fips_provider"}
// Dependencies: {}
# [doc = " This function returns a [`CryptoProvider`] that uses"] # [doc = " FIPS140-3-approved cryptography."] # [doc = ""] # [doc = " Using this function expresses in your code that you require"] # [doc = " FIPS-approved cryptography, and will not compile if you make"] # [doc = " a mistake with cargo features."] # [doc = ""] # [doc = " See our [FIPS documentation](crate::manual::_06_fips) for"] # [doc = " more detail."] # [doc = ""] # [doc = " Install this as the process-default provider, like:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(feature = \"fips\")] {"] # [doc = " rustls::crypto::default_fips_provider().install_default()"] # [doc = "     .expect(\"default provider already set elsewhere\");"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " You can also use this explicitly, like:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(feature = \"fips\")] {"] # [doc = " # let root_store = rustls::RootCertStore::empty();"] # [doc = " let config = rustls::ClientConfig::builder("] # [doc = "         rustls::crypto::default_fips_provider().into()"] # [doc = "     )"] # [doc = "     .with_root_certificates(root_store)"] # [doc = "     .with_no_client_auth()"] # [doc = "     .unwrap();"] # [doc = " # }"] # [doc = " ```"] # [cfg (all (feature = "aws-lc-rs" , any (feature = "fips" , rustls_docsrs)))] # [cfg_attr (rustls_docsrs , doc (cfg (feature = "fips")))] pub fn default_fips_provider () -> CryptoProvider { aws_lc_rs :: DEFAULT_PROVIDER }
};
}
