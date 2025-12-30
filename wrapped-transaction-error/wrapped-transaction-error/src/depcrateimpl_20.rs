// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl core :: error :: Error for TransportError { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { TransportError :: IoError (e) => Some (e) , TransportError :: TransactionError (e) => Some (e) , TransportError :: Custom (_) => None , } } }
};
}
