// Generated macro for Pending (struct)
macro_rules! Depcrate_iterator_backendPending {
() => {
// Module: crate::iterator::backend
// Provides: {"Pending"}
// Dependencies: {}
# [doc = " The iterator of one batch of signals."] # [doc = ""] # [doc = " This is returned by the [`pending`][SignalDelivery::pending] method."] # [derive (Debug)] pub struct Pending < E : Exfiltrator > { pending : Arc < PendingSignals < E > > , position : usize , }
};
}
