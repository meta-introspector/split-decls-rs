// Generated macro for max (function)
macro_rules! Depcrate_utilmax {
() => {
// Module: crate::util
// Provides: {"max"}
// Dependencies: {}
pub (crate) const fn max (a : NonZeroUsize , b : NonZeroUsize) -> NonZeroUsize { if a . get () < b . get () { b } else { a } }
};
}
