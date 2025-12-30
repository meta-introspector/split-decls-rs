// Generated macro for impl_859 (impl)
macro_rules! Depcrate_quickcheckimpl_859 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_859"}
// Dependencies: {}
impl Arbitrary for UtcDateTime { # [inline] fn arbitrary (g : & mut Gen) -> Self { Self :: new (< _ > :: arbitrary (g) , < _ > :: arbitrary (g)) } # [inline] fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { Box :: new ((self . date () , self . time ()) . shrink () . map (| (date , time) | Self :: new (date , time)) ,) } }
};
}
