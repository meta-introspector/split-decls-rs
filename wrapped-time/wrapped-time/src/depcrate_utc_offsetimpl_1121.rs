// Generated macro for impl_1121 (impl)
macro_rules! Depcrate_utc_offsetimpl_1121 {
() => {
// Module: crate::utc_offset
// Provides: {"impl_1121"}
// Dependencies: {}
impl Neg for UtcOffset { type Output = Self ; # [inline] fn neg (self) -> Self :: Output { Self :: from_hms_ranged (self . hours . neg () , self . minutes . neg () , self . seconds . neg ()) } }
};
}
