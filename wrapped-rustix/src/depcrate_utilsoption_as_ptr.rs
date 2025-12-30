// Generated macro for option_as_ptr (function)
macro_rules! Depcrate_utilsoption_as_ptr {
() => {
// Module: crate::utils
// Provides: {"option_as_ptr"}
// Dependencies: {}
# [doc = " Convert an `Option<&T>` into a possibly-null `*const T`."] # [inline] pub (crate) const fn option_as_ptr < T > (t : Option < & T >) -> * const T { match t { Some (t) => t , None => null () , } }
};
}
