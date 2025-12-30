// Generated macro for SignalDelivery (struct)
macro_rules! Depcrate_iterator_backendSignalDelivery {
() => {
// Module: crate::iterator::backend
// Provides: {"SignalDelivery"}
// Dependencies: {}
# [doc = " A struct for delivering received signals to the main program flow."] # [doc = " The self-pipe IO type is generic. See the"] # [doc = " [`with_pipe`][SignalDelivery::with_pipe] method for requirements"] # [doc = " for the IO type."] # [derive (Debug)] pub struct SignalDelivery < R , E : Exfiltrator > { read : R , handle : Handle , pending : Arc < PendingSignals < E > > , }
};
}
