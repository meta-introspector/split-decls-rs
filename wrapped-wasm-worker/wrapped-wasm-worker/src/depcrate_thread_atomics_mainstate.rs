// Generated macro for State (struct)
macro_rules! Depcrate_thread_atomics_mainState {
() => {
// Module: crate::thread::atomics::main
// Provides: {"State"}
// Dependencies: {}
# [doc = " State for each [`Worker`]."] pub (super) struct State { # [doc = " [`Worker`]"] pub (super) this : Worker , # [doc = " Callback handling messages."] # [cfg (feature = "message")] pub (super) _message_handler : Closure < dyn Fn (MessageEvent) > , }
};
}
