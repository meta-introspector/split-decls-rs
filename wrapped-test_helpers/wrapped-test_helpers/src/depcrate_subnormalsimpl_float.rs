// Generated macro for impl_float (macro)
macro_rules! Depcrate_subnormalsimpl_float {
() => {
// Module: crate::subnormals
// Provides: {"impl_float"}
// Dependencies: {}
macro_rules ! impl_float { { $ ($ ty : ty) ,* } => { $ (impl FlushSubnormals for $ ty { fn flush (self) -> Self { let is_f32 = size_of ::< Self > () == 4 ; let ppc_flush = is_f32 && cfg ! (all (any (target_arch = "powerpc" , all (target_arch = "powerpc64" , target_endian = "big")) , target_feature = "altivec" , not (target_feature = "vsx") ,)) ; let arm_flush = is_f32 && cfg ! (all (target_arch = "arm" , target_feature = "neon")) ; let flush = ppc_flush || arm_flush ; if flush && self . is_subnormal () { <$ ty >:: copysign (0. , self) } else { self } } }) * } }
};
}
