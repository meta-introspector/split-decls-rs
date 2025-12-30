// Generated macro for opt_fn (macro)
macro_rules! Depcrate_util_macrosopt_fn {
() => {
// Module: crate::util::macros
// Provides: {"opt_fn"}
// Dependencies: {}
# [doc = " Expands to an `Option<fn>` type with the given argument types and return"] # [doc = " type. Designed for use with `unsafe_impl_for_power_set`."] macro_rules ! opt_fn { ($ ($ args : ident) ,* -> $ ret : ident) => { Option < fn ($ ($ args) ,*) -> $ ret > } ; }
};
}
