// Generated macro for deflate_bound_correct (function)
macro_rules! Depcrate_deflatedeflate_bound_correct {
() => {
// Module: crate::deflate
// Provides: {"deflate_bound_correct"}
// Dependencies: {}
# [test] # [cfg_attr (miri , ignore = "slow")] fn deflate_bound_correct () { :: quickcheck :: quickcheck (test as fn (_) -> _) ; fn test (input : (DeflateConfig , c_ulong)) -> bool { assert_deflate_bound_correct (input) ; true } }
};
}
