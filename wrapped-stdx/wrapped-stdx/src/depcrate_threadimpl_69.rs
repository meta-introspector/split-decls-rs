// Generated macro for impl_69 (impl)
macro_rules! Depcrate_threadimpl_69 {
() => {
// Module: crate::thread
// Provides: {"impl_69"}
// Dependencies: {}
impl < T > JoinHandle < T > { # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if there is no thread to join."] # [must_use] pub fn join (mut self) -> T { self . inner . take () . unwrap () . join () } }
};
}
