// Generated macro for impl_544 (impl)
macro_rules! Depcrate_zerovecimpl_544 {
() => {
// Module: crate::zerovec
// Provides: {"impl_544"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < U > Drop for EyepatchHackVector < U > { # [inline] fn drop (& mut self) { if self . capacity != 0 { unsafe { let _ = self . get_vec () ; } } } }
};
}
