// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl From < SanitizeMessageError > for TransactionError { fn from (err : SanitizeMessageError) -> Self { match err { SanitizeMessageError :: AddressLoaderError (err) => Self :: from (err) , _ => Self :: SanitizeFailure , } } }
};
}
