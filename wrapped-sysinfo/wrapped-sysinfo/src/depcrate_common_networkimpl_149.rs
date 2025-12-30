// Generated macro for impl_149 (impl)
macro_rules! Depcrate_common_networkimpl_149 {
() => {
// Module: crate::common::network
// Provides: {"impl_149"}
// Dependencies: {}
impl MacAddr { # [doc = " A `MacAddr` with all bytes set to `0`."] pub const UNSPECIFIED : Self = MacAddr ([0 ; 6]) ; # [doc = " Checks if this `MacAddr` has all bytes equal to `0`."] pub fn is_unspecified (& self) -> bool { self == & MacAddr :: UNSPECIFIED } }
};
}
