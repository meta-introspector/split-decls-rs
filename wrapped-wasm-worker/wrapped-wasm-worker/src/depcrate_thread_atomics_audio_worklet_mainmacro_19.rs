// Generated macro for macro_19 (macro)
macro_rules! Depcrate_thread_atomics_audio_worklet_mainmacro_19 {
() => {
// Module: crate::thread::atomics::audio_worklet::main
// Provides: {"macro_19"}
// Dependencies: {}
thread_local ! { # [doc = " Containing all spawned audio worklets."] pub (super) static WORKLETS : RefCell < HashMap < ThreadId , State >> = RefCell :: new (HashMap :: new ()) ; }
};
}
