// Generated macro for in_buggy_qemu (function)
macro_rules! Depcrate_subnormalsin_buggy_qemu {
() => {
// Module: crate::subnormals
// Provides: {"in_buggy_qemu"}
// Dependencies: {}
# [doc = " AltiVec should flush subnormal inputs to zero, but QEMU seems to only flush outputs."] # [doc = " https://gitlab.com/qemu-project/qemu/-/issues/1779"] # [cfg (all (any (target_arch = "powerpc" , target_arch = "powerpc64") , target_feature = "altivec"))] fn in_buggy_qemu () -> bool { use std :: sync :: OnceLock ; static BUGGY : OnceLock < bool > = OnceLock :: new () ; fn add (x : f32 , y : f32) -> f32 { # [cfg (target_arch = "powerpc")] use core :: arch :: powerpc :: * ; # [cfg (target_arch = "powerpc64")] use core :: arch :: powerpc64 :: * ; let array : [f32 ; 4] = unsafe { core :: mem :: transmute (vec_add (vec_splats (x) , vec_splats (y))) } ; array [0] } * BUGGY . get_or_init (| | add (- 1.0857398e-38 , 0.) . is_sign_negative ()) }
};
}
