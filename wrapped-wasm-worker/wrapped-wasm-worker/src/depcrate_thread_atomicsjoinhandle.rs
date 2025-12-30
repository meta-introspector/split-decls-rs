// Generated macro for JoinHandle (struct)
macro_rules! Depcrate_thread_atomicsJoinHandle {
() => {
// Module: crate::thread::atomics
// Provides: {"JoinHandle"}
// Dependencies: {}
# [doc = " Implementation of [`std::thread::JoinHandle`]."] pub (super) struct JoinHandle < T > { # [doc = " Receiver for the return value."] receiver : Option < Receiver < T > > , # [doc = " Corresponding [`Thread`]."] thread : Thread , }
};
}
