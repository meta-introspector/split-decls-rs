// Generated macro for SupportedProtocolVersion (enum)
macro_rules! Depcrate_versionsSupportedProtocolVersion {
() => {
// Module: crate::versions
// Provides: {"SupportedProtocolVersion"}
// Dependencies: {}
# [doc = " A TLS protocol version supported by rustls."] # [doc = ""] # [doc = " All possible values of this enum are provided by the library in"] # [doc = " the [`ALL_VERSIONS`] array, as well as individually as [`TLS12`]"] # [doc = " and [`TLS13`]."] # [non_exhaustive] # [derive (Debug)] pub enum SupportedProtocolVersion { # [doc = " The TLS1.2 protocol version."] TLS12 (& 'static Tls12Version) , # [doc = " The TLS1.3 protocol version."] TLS13 (& 'static Tls13Version) , }
};
}
