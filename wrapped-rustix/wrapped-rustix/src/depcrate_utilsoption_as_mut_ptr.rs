// Generated macro for option_as_mut_ptr (function)
macro_rules! Depcrate_utilsoption_as_mut_ptr {
() => {
// Module: crate::utils
// Provides: {"option_as_mut_ptr"}
// Dependencies: {}
# [doc = " Convert an `Option<&mut T>` into a possibly-null `*mut T`."] # [inline] pub (crate) fn option_as_mut_ptr < T > (t : Option < & mut T >) -> * mut T { match t { Some (t) => t , None => null_mut () , } }
};
}
