// Generated macro for lgammaf_r (function)
macro_rules! Depcrate_sys_cmathlgammaf_r {
() => {
// Module: crate::sys::cmath
// Provides: {"lgammaf_r"}
// Dependencies: {}
# [cfg (target_os = "aix")] pub fn lgammaf_r (n : f32 , s : & mut i32) -> f32 { lgamma_r (n . into () , s) as f32 }
};
}
