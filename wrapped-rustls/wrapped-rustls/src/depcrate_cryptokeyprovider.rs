// Generated macro for KeyProvider (trait)
macro_rules! Depcrate_cryptoKeyProvider {
() => {
// Module: crate::crypto
// Provides: {"KeyProvider"}
// Dependencies: {}
# [doc = " A mechanism for loading private [`SigningKey`]s from [`PrivateKeyDer`]."] # [doc = ""] # [doc = " This trait is intended to be used with private key material that is sourced from DER,"] # [doc = " such as a private-key that may be present on-disk. It is not intended to be used with"] # [doc = " keys held in hardware security modules (HSMs) or physical tokens. For these use-cases"] # [doc = " see the Rustls manual section on [customizing private key usage]."] # [doc = ""] # [doc = " [customizing private key usage]: <https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#customising-private-key-usage>"] pub trait KeyProvider : Send + Sync + Debug { # [doc = " Decode and validate a private signing key from `key_der`."] # [doc = ""] # [doc = " This is used by [`ConfigBuilder::with_client_auth_cert()`], [`ConfigBuilder::with_single_cert()`],"] # [doc = " and [`ConfigBuilder::with_single_cert_with_ocsp()`].  The key types and formats supported by this"] # [doc = " function directly defines the key types and formats supported in those APIs."] # [doc = ""] # [doc = " Return an error if the key type encoding is not supported, or if the key fails validation."] fn load_private_key (& self , key_der : PrivateKeyDer < 'static > ,) -> Result < Box < dyn SigningKey > , Error > ; # [doc = " Return `true` if this is backed by a FIPS-approved implementation."] # [doc = ""] # [doc = " If this returns `true`, that must be the case for all possible key types"] # [doc = " supported by [`KeyProvider::load_private_key()`]."] fn fips (& self) -> bool { false } }
};
}
