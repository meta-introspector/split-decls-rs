// Generated macro for impl_857 (impl)
macro_rules! Depcrate_quickcheckimpl_857 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_857"}
// Dependencies: {}
impl Arbitrary for UtcOffset { # [inline] fn arbitrary (g : & mut Gen) -> Self { Self :: from_hms_ranged (< _ > :: arbitrary (g) , < _ > :: arbitrary (g) , < _ > :: arbitrary (g)) } # [inline] fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { Box :: new (self . as_hms_ranged () . shrink () . map (| (hours , minutes , seconds) | Self :: from_hms_ranged (hours , minutes , seconds)) ,) } }
};
}
