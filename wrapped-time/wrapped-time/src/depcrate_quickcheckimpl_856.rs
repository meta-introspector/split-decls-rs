// Generated macro for impl_856 (impl)
macro_rules! Depcrate_quickcheckimpl_856 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_856"}
// Dependencies: {}
impl Arbitrary for PrimitiveDateTime { # [inline] fn arbitrary (g : & mut Gen) -> Self { Self :: new (< _ > :: arbitrary (g) , < _ > :: arbitrary (g)) } # [inline] fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { Box :: new ((self . date () , self . time ()) . shrink () . map (| (date , time) | Self :: new (date , time)) ,) } }
};
}
