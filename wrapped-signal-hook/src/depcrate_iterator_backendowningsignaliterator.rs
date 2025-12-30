// Generated macro for OwningSignalIterator (type)
macro_rules! Depcrate_iterator_backendOwningSignalIterator {
() => {
// Module: crate::iterator::backend
// Provides: {"OwningSignalIterator"}
// Dependencies: {}
# [doc = " A signal iterator which consumes a [`SignalDelivery`] instance and takes"] # [doc = " ownership of it."] pub type OwningSignalIterator < R , E > = SignalIterator < SignalDelivery < R , E > , E > ;
};
}
