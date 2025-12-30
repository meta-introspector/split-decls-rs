// Generated macro for KeyBuilder (struct)
macro_rules! Depcrate_quicKeyBuilder {
() => {
// Module: crate::quic
// Provides: {"KeyBuilder"}
// Dependencies: {}
pub (crate) struct KeyBuilder < 'a > { expander : Box < dyn HkdfExpander > , version : Version , alg : & 'a dyn Algorithm , }
};
}
