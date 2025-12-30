// Generated macro for impl_21 (impl)
macro_rules! Depcrate_de_valueimpl_21 {
() => {
// Module: crate::de::value
// Provides: {"impl_21"}
// Dependencies: {}
impl de :: Error for Error { # [cfg (any (feature = "std" , feature = "alloc"))] # [cold] fn custom < T > (msg : T) -> Self where T : Display , { Error { err : msg . to_string () . into_boxed_str () , } } # [cfg (not (any (feature = "std" , feature = "alloc")))] # [cold] fn custom < T > (msg : T) -> Self where T : Display , { let _ = msg ; Error { err : () } } }
};
}
