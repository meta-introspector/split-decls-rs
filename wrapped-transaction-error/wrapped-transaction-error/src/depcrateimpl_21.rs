// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl fmt :: Display for TransportError { fn fmt (& self , f : & mut fmt :: Formatter) -> :: core :: fmt :: Result { match self { Self :: IoError (e) => f . write_fmt (format_args ! ("transport io error: {e}")) , Self :: TransactionError (e) => { f . write_fmt (format_args ! ("transport transaction error: {e}")) } Self :: Custom (s) => f . write_fmt (format_args ! ("transport custom error: {s}")) , } } }
};
}
