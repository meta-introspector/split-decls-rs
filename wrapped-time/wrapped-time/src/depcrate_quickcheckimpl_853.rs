// Generated macro for impl_853 (impl)
macro_rules! Depcrate_quickcheckimpl_853 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_853"}
// Dependencies: {}
impl Arbitrary for Date { # [inline] fn arbitrary (g : & mut Gen) -> Self { unsafe { Self :: from_julian_day_unchecked (arbitrary_between ! (i32 ; g , Self :: MIN . to_julian_day () , Self :: MAX . to_julian_day ())) } } # [inline] fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { Box :: new (self . to_ordinal_date () . shrink () . flat_map (| (year , ordinal) | Self :: from_ordinal_date (year , ordinal)) ,) } }
};
}
