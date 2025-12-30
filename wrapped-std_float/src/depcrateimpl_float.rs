// Generated macro for impl_float (macro)
macro_rules! Depcrateimpl_float {
() => {
// Module: crate
// Provides: {"impl_float"}
// Dependencies: {}
macro_rules ! impl_float { { $ ($ fn : ident : $ intrinsic : ident ,) * } => { impl < const N : usize > StdFloat for Simd < f32 , N > where LaneCount < N >: SupportedLaneCount , { # [inline] fn fract (self) -> Self { self - self . trunc () } $ (# [inline] fn $ fn (self) -> Self { unsafe { intrinsics ::$ intrinsic (self) } }) * } impl < const N : usize > StdFloat for Simd < f64 , N > where LaneCount < N >: SupportedLaneCount , { # [inline] fn fract (self) -> Self { self - self . trunc () } $ (# [inline] fn $ fn (self) -> Self { # [cfg (target_arch = "aarch64")] { let mut ln = Self :: splat (0f64) ; for i in 0 .. N { ln [i] = self [i] .$ fn () } ln } # [cfg (not (target_arch = "aarch64"))] { unsafe { intrinsics ::$ intrinsic (self) } } }) * } } }
};
}
