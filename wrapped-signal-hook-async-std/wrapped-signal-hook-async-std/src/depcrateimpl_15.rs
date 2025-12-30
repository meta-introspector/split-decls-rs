// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < E : Exfiltrator > SignalsInfo < E > { fn has_signals (read : & mut Async < UnixStream > , ctx : & mut Context < '_ >) -> Result < bool , Error > { match Pin :: new (read) . poll_read (ctx , & mut [0u8]) { Poll :: Pending => Ok (false) , Poll :: Ready (Ok (num_read)) => Ok (num_read > 0) , Poll :: Ready (Err (error)) => Err (error) , } } }
};
}
