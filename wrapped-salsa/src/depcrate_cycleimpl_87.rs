// Generated macro for impl_87 (impl)
macro_rules! Depcrate_cycleimpl_87 {
() => {
// Module: crate::cycle
// Provides: {"impl_87"}
// Dependencies: {}
impl AtomicIterationCount { pub (crate) fn load (& self) -> IterationCount { IterationCount (self . 0 . load (Ordering :: Relaxed)) } pub (crate) fn load_mut (& mut self) -> IterationCount { IterationCount (* self . 0 . get_mut ()) } pub (crate) fn store (& self , value : IterationCount) { self . 0 . store (value . 0 , Ordering :: Release) ; } pub (crate) fn store_mut (& mut self , value : IterationCount) { * self . 0 . get_mut () = value . 0 ; } }
};
}
