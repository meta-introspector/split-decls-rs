// Generated macro for RefSignalIterator (type)
macro_rules! Depcrate_iterator_backendRefSignalIterator {
() => {
// Module: crate::iterator::backend
// Provides: {"RefSignalIterator"}
// Dependencies: {}
# [doc = " A signal iterator which takes a mutable reference to a [`SignalDelivery`]"] # [doc = " instance."] pub type RefSignalIterator < 'a , R , E > = SignalIterator < & 'a mut SignalDelivery < R , E > , E > ;
};
}
