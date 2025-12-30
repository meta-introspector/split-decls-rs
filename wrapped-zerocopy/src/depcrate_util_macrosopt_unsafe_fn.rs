// Generated macro for opt_unsafe_fn (macro)
macro_rules! Depcrate_util_macrosopt_unsafe_fn {
() => {
// Module: crate::util::macros
// Provides: {"opt_unsafe_fn"}
// Dependencies: {}
# [doc = " Expands to an `Option<unsafe fn>` type with the given argument types and"] # [doc = " return type. Designed for use with `unsafe_impl_for_power_set`."] macro_rules ! opt_unsafe_fn { ($ ($ args : ident) ,* -> $ ret : ident) => { Option < unsafe fn ($ ($ args) ,*) -> $ ret > } ; }
};
}
