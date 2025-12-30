// Generated macro for impl_916 (impl)
macro_rules! Depcrate_combinator_debug_internalsimpl_916 {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"impl_916"}
// Dependencies: {}
impl Depth { pub (crate) fn new () -> Self { let depth = DEPTH . fetch_add (1 , std :: sync :: atomic :: Ordering :: SeqCst) ; let inc = true ; Self { depth , inc } } pub (crate) fn existing () -> Self { let depth = DEPTH . load (std :: sync :: atomic :: Ordering :: SeqCst) ; let inc = false ; Self { depth , inc } } }
};
}
