// Generated macro for State (struct)
macro_rules! Depcrate_thread_atomics_audio_worklet_mainState {
() => {
// Module: crate::thread::atomics::audio_worklet::main
// Provides: {"State"}
// Dependencies: {}
# [doc = " State for each audio worklet."] pub (super) struct State { # [doc = " [`MessagePort`]"] pub (super) port : MessagePort , # [doc = " Callback handling messages."] pub (super) _message_handler : Closure < dyn Fn (MessageEvent) > , }
};
}
