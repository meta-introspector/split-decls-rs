// Generated macro for init_main_thread (function)
macro_rules! Depcrate_thread_atomics_audio_worklet_maininit_main_thread {
() => {
// Module: crate::thread::atomics::audio_worklet::main
// Provides: {"init_main_thread"}
// Dependencies: {}
# [doc = " Initializes the main thread worklet handler. Make sure to call this at"] # [doc = " least once on the main thread before spawning any audio worklet."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This will panic if called outside the main thread."] pub (in super :: super) fn init_main_thread () { debug_assert ! (super :: super :: is_main_thread () , "initizalizing main thread without being on the main thread") ; DESTROY_SENDER . get_or_init (| | { let (sender , receiver) = channel :: channel () ; wasm_bindgen_futures :: spawn_local (async move { while let Ok (id) = receiver . next () . await { let state = WORKLETS . with (| worklets | { worklets . borrow_mut () . remove (& id) . expect ("audio worklet to be terminated not found") }) ; state . port . set_onmessage (None) ; } }) ; sender }) ; }
};
}
