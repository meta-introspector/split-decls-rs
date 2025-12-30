// Generated macro for impl_125 (impl)
macro_rules! Depcrate_signing_keyimpl_125 {
() => {
// Module: crate::signing_key
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl < P : ParameterSet > Drop for SigningKey < P > { fn drop (& mut self) { self . sk_seed . 0 . zeroize () ; self . sk_prf . 0 . zeroize () ; } }
};
}
