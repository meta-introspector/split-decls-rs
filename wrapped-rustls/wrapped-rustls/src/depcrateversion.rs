// Generated macro for version (module)
macro_rules! Depcrateversion {
() => {
// Module: crate
// Provides: {"version"}
// Dependencies: {}
# [doc = " All defined protocol versions appear in this module."] # [doc = ""] # [doc = " ALL_VERSIONS is a provided as an array of all of these values."] pub mod version { pub use crate :: versions :: { TLS12 , TLS12_VERSION , TLS13 , TLS13_VERSION , Tls12Version , Tls13Version , } ; }
};
}
