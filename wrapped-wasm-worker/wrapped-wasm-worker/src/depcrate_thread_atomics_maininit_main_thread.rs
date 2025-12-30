// Generated macro for init_main_thread (function)
macro_rules! Depcrate_thread_atomics_maininit_main_thread {
() => {
// Module: crate::thread::atomics::main
// Provides: {"init_main_thread"}
// Dependencies: {}
# [doc = " Initializes the main thread worker handler. Make sure to call this at"] # [doc = " least once on the main thread before spawning any thread."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This will panic if called outside the main thread."] pub (super) fn init_main_thread () { debug_assert ! (super :: is_main_thread () , "initizalizing main thread without being on the main thread") ; COMMAND_SENDER . get_or_init (| | { super :: has_spawn_support () ; let (sender , receiver) = channel :: channel :: < Command > () ; wasm_bindgen_futures :: spawn_local (async move { while let Ok (command) = receiver . next () . await { match command { Command :: Spawn (SpawnData { id , name , stack_size , # [cfg (feature = "message")] spawn_receiver , task , }) => { spawn :: spawn_internal (id , name . as_deref () , stack_size , # [cfg (feature = "message")] spawn_receiver , Box :: new (task) ,) ; } Command :: Terminate { id , value , memory } => { wasm_bindgen_futures :: spawn_local (async move { WaitAsync :: wait (& value , 0) . await ; unsafe { memory . release () } . expect ("attempted to clean up main thread") ; let state = WORKERS . with (| workers | { workers . borrow_mut () . remove (& id) . expect ("`Worker` to be terminated not found") }) ; state . this . terminate () ; # [cfg (feature = "message")] state . this . set_onmessage (None) ; }) ; } } } }) ; # [cfg (all (feature = "audio-worklet" , feature = "message"))] super :: audio_worklet :: main :: init_main_thread () ; sender }) ; }
};
}
