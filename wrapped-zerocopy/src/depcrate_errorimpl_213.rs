// Generated macro for impl_213 (impl)
macro_rules! Depcrate_errorimpl_213 {
() => {
// Module: crate::error
// Provides: {"impl_213"}
// Dependencies: {}
# [cfg (test)] impl < Src , Dst > AlignmentError < Src , Dst > { fn new_checked (src : Src) -> AlignmentError < Src , Dst > { assert_ne ! (core :: mem :: align_of ::< Dst > () , 1) ; unsafe { AlignmentError :: new_unchecked (src) } } }
};
}
