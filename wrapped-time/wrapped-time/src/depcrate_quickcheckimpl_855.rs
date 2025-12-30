// Generated macro for impl_855 (impl)
macro_rules! Depcrate_quickcheckimpl_855 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_855"}
// Dependencies: {}
impl Arbitrary for Time { # [inline] fn arbitrary (g : & mut Gen) -> Self { Self :: from_hms_nanos_ranged (< _ > :: arbitrary (g) , < _ > :: arbitrary (g) , < _ > :: arbitrary (g) , < _ > :: arbitrary (g) ,) } # [inline] fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { Box :: new (self . as_hms_nano_ranged () . shrink () . map (| (hour , minute , second , nanosecond) | { Self :: from_hms_nanos_ranged (hour , minute , second , nanosecond) }) ,) } }
};
}
