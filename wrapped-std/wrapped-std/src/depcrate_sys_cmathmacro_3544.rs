// Generated macro for macro_3544 (macro)
macro_rules! Depcrate_sys_cmathmacro_3544 {
() => {
// Module: crate::sys::cmath
// Provides: {"macro_3544"}
// Dependencies: {}
cfg_select ! { all (target_os = "windows" , target_env = "msvc" , target_arch = "x86") => { # [inline] pub fn acosf (n : f32) -> f32 { f64 :: acos (n as f64) as f32 } # [inline] pub fn asinf (n : f32) -> f32 { f64 :: asin (n as f64) as f32 } # [inline] pub fn atan2f (n : f32 , b : f32) -> f32 { f64 :: atan2 (n as f64 , b as f64) as f32 } # [inline] pub fn atanf (n : f32) -> f32 { f64 :: atan (n as f64) as f32 } # [inline] pub fn coshf (n : f32) -> f32 { f64 :: cosh (n as f64) as f32 } # [inline] pub fn sinhf (n : f32) -> f32 { f64 :: sinh (n as f64) as f32 } # [inline] pub fn tanf (n : f32) -> f32 { f64 :: tan (n as f64) as f32 } # [inline] pub fn tanhf (n : f32) -> f32 { f64 :: tanh (n as f64) as f32 } } _ => { unsafe extern "C" { pub safe fn acosf (n : f32) -> f32 ; pub safe fn asinf (n : f32) -> f32 ; pub safe fn atan2f (a : f32 , b : f32) -> f32 ; pub safe fn atanf (n : f32) -> f32 ; pub safe fn coshf (n : f32) -> f32 ; pub safe fn sinhf (n : f32) -> f32 ; pub safe fn tanf (n : f32) -> f32 ; pub safe fn tanhf (n : f32) -> f32 ; } } }
};
}
