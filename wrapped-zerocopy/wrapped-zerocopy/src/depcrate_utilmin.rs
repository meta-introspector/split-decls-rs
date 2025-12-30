// Generated macro for min (function)
macro_rules! Depcrate_utilmin {
() => {
// Module: crate::util
// Provides: {"min"}
// Dependencies: {}
pub (crate) const fn min (a : NonZeroUsize , b : NonZeroUsize) -> NonZeroUsize { if a . get () > b . get () { b } else { a } }
};
}
