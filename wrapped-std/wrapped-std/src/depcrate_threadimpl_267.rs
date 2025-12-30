// Generated macro for impl_267 (impl)
macro_rules! Depcrate_threadimpl_267 {
() => {
// Module: crate::thread
// Provides: {"impl_267"}
// Dependencies: {}
impl < 'scope , T > JoinInner < 'scope , T > { fn join (mut self) -> Result < T > { self . native . join () ; Arc :: get_mut (& mut self . packet) . expect ("threads should not terminate unexpectedly") . result . get_mut () . take () . unwrap () } }
};
}
