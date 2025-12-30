// Generated macro for impl_858 (impl)
macro_rules! Depcrate_quickcheckimpl_858 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_858"}
// Dependencies: {}
impl Arbitrary for OffsetDateTime { # [inline] fn arbitrary (g : & mut Gen) -> Self { Self :: new_in_offset (< _ > :: arbitrary (g) , < _ > :: arbitrary (g) , < _ > :: arbitrary (g)) } # [inline] fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { Box :: new ((self . date () , self . time () , self . offset ()) . shrink () . map (| (date , time , offset) | Self :: new_in_offset (date , time , offset)) ,) } }
};
}
