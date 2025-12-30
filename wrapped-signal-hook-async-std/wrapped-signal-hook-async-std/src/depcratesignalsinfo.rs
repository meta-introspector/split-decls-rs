// Generated macro for SignalsInfo (struct)
macro_rules! DepcrateSignalsInfo {
() => {
// Module: crate
// Provides: {"SignalsInfo"}
// Dependencies: {}
# [doc = " An asynchronous [`Stream`] of arriving signals."] # [doc = ""] # [doc = " The stream doesn't return the signals in the order they were recieved by"] # [doc = " the process and may merge signals received multiple times."] pub struct SignalsInfo < E : Exfiltrator = SignalOnly > (OwningSignalIterator < Async < UnixStream > , E >) ;
};
}
