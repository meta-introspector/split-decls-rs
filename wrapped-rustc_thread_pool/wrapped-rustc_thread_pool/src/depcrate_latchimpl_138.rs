// Generated macro for impl_138 (impl)
macro_rules! Depcrate_latchimpl_138 {
() => {
// Module: crate::latch
// Provides: {"impl_138"}
// Dependencies: {}
impl < L : Latch > Latch for LatchRef < '_ , L > { # [inline] unsafe fn set (this : * const Self) { unsafe { L :: set ((* this) . inner) } ; } }
};
}
