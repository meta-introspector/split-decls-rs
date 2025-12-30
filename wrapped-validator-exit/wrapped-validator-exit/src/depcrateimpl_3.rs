// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl Exit { pub fn register_exit (& mut self , exit : Box < dyn FnOnce () + Send + Sync >) { if self . exited { exit () ; } else { self . exits . push (exit) ; } } pub fn exit (& mut self) { self . exited = true ; for exit in self . exits . drain (..) { exit () ; } } }
};
}
