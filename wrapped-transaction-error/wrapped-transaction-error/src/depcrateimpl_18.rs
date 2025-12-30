// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl From < SanitizeError > for SanitizeMessageError { fn from (err : SanitizeError) -> Self { match err { SanitizeError :: IndexOutOfBounds => Self :: IndexOutOfBounds , SanitizeError :: ValueOutOfBounds => Self :: ValueOutOfBounds , SanitizeError :: InvalidValue => Self :: InvalidValue , } } }
};
}
