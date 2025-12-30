// Generated macro for spawn (function)
macro_rules! Depcrate_thread_atomics_spawnspawn {
() => {
// Module: crate::thread::atomics::spawn
// Provides: {"spawn"}
// Dependencies: {}
# [doc = " Internal spawn function."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `task` has to outlive the thread."] # [allow (clippy :: unnecessary_wraps)] pub (super) unsafe fn spawn < F1 , F2 , T > (task : F1 , name : Option < String > , stack_size : Option < usize > , scope : Option < Arc < ScopeData > > ,) -> io :: Result < JoinHandle < T > > where F1 : FnOnce () -> F2 + Send , F2 : Future < Output = T > , T : Send , { let thread = thread_init (name , scope . as_deref ()) ; let (result_sender , result_receiver) = oneshot :: channel () ; # [cfg (feature = "message")] let (spawn_sender , spawn_receiver) = channel :: channel () ; let task : Task < '_ > = Box :: new ({ let thread = thread . clone () ; move | _ | { thread_runner (thread , stack_size , result_sender , # [cfg (feature = "message")] spawn_sender , scope , task ,) } }) ; Ok (spawn_without_message (thread , stack_size , result_receiver , # [cfg (feature = "message")] spawn_receiver , task ,)) }
};
}
