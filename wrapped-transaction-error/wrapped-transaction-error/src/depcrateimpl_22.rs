// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl From < std :: io :: Error > for TransportError { fn from (e : std :: io :: Error) -> Self { TransportError :: IoError (e) } }
};
}
