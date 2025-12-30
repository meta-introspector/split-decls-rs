// Generated macro for SignalIterator (struct)
macro_rules! Depcrate_iterator_backendSignalIterator {
() => {
// Module: crate::iterator::backend
// Provides: {"SignalIterator"}
// Dependencies: {}
# [doc = " An infinite iterator of received signals."] pub struct SignalIterator < SD , E : Exfiltrator > { signals : SD , iter : Pending < E > , }
};
}
