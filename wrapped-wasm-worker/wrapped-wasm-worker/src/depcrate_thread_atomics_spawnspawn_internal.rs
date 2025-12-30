// Generated macro for spawn_internal (function)
macro_rules! Depcrate_thread_atomics_spawnspawn_internal {
() => {
// Module: crate::thread::atomics::spawn
// Provides: {"spawn_internal"}
// Dependencies: {}
# [doc = " Spawning thread regardless of being nested."] pub (super) fn spawn_internal (id : ThreadId , name : Option < & str > , stack_size : Option < usize > , # [cfg (feature = "message")] spawn_receiver : channel :: Receiver < SpawnData > , task : Task < '_ > ,) { spawn_common (id , name , # [cfg (feature = "message")] spawn_receiver , task , | worker , module , memory , task | { # [cfg (not (feature = "audio-worklet"))] let message = Array :: of4 (module , memory , & stack_size . into () , & task) ; # [cfg (feature = "audio-worklet")] let message = { THREAD_LOCK_INDEXES . with (| indexes | Array :: of5 (module , memory , & stack_size . into () , indexes , & task)) } ; worker . post_message (& message) } ,) . expect ("`Worker.postMessage()` is not expected to fail without a `transfer` object") ; }
};
}
