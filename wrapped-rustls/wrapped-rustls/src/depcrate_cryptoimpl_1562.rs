// Generated macro for impl_1562 (impl)
macro_rules! Depcrate_cryptoimpl_1562 {
() => {
// Module: crate::crypto
// Provides: {"impl_1562"}
// Dependencies: {}
impl CryptoProvider { # [doc = " Sets this `CryptoProvider` as the default for this process."] # [doc = ""] # [doc = " This can be called successfully at most once in any process execution."] # [doc = ""] # [doc = " Call this early in your process to configure which provider is used for"] # [doc = " the provider.  The configuration should happen before any use of"] # [doc = " [`ClientConfig::builder()`] or [`ServerConfig::builder()`]."] pub fn install_default (self) -> Result < () , Arc < Self > > { static_default :: install_default (self) } }
};
}
