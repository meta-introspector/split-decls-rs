// Generated macro for impl_922 (impl)
macro_rules! Depcrate_combinator_debug_internalsimpl_922 {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"impl_922"}
// Dependencies: {}
impl Severity { pub (crate) fn with_result < T , I : Stream , E : ParserError < I > > (result : & Result < T , E >) -> Self { match result { Ok (_) => Self :: Success , Err (e) if e . is_backtrack () => Self :: Backtrack , Err (e) if e . is_incomplete () => Self :: Incomplete , _ => Self :: Cut , } } }
};
}
