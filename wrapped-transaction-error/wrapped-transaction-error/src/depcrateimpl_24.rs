// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl TransportError { pub fn unwrap (& self) -> TransactionError { if let TransportError :: TransactionError (err) = self { err . clone () } else { panic ! ("unexpected transport error") } } }
};
}
