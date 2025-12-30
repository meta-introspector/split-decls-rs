// Generated macro for WakerData (struct)
macro_rules! Depcrate_thread_yield_nowWakerData {
() => {
// Module: crate::thread::yield_now
// Provides: {"WakerData"}
// Dependencies: {}
# [doc = " Data required to wake up the [`Future`]."] # [derive (Debug)] struct WakerData { # [doc = " Represents if it has completed or not."] completed : bool , # [doc = " Stores the [`Waker`]."] waker : Option < Waker > , }
};
}
