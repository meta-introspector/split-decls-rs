// Generated macro for impl_442 (impl)
macro_rules! Depcrate_split_atimpl_442 {
() => {
// Module: crate::split_at
// Provides: {"impl_442"}
// Dependencies: {}
impl < T > Split < T > { # [doc = " Produces a `Split` of `source` with `l_len`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `l_len` is no greater than `source`'s length."] # [inline (always)] unsafe fn new (source : T , l_len : usize) -> Self { Self { source , l_len } } }
};
}
