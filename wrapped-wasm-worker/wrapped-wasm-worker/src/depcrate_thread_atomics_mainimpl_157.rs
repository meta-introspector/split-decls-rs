// Generated macro for impl_157 (impl)
macro_rules! Depcrate_thread_atomics_mainimpl_157 {
() => {
// Module: crate::thread::atomics::main
// Provides: {"impl_157"}
// Dependencies: {}
impl Command { # [doc = " Sends command to be executed on the main thread."] pub (super) fn send (self) { COMMAND_SENDER . get () . expect ("sending `Command` before `COMMAND_SENDER` is initialized") . send (self) . expect ("`Receiver` was somehow dropped from the main thread") ; } }
};
}
