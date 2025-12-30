// Generated macro for macro_154 (macro)
macro_rules! Depcrate_thread_atomics_mainmacro_154 {
() => {
// Module: crate::thread::atomics::main
// Provides: {"macro_154"}
// Dependencies: {}
thread_local ! { # [doc = " Containing all spawned workers."] pub (super) static WORKERS : RefCell < HashMap < ThreadId , State >> = RefCell :: new (HashMap :: new ()) ; }
};
}
