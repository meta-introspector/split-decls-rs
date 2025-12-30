// Generated macro for impl_854 (impl)
macro_rules! Depcrate_quickcheckimpl_854 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_854"}
// Dependencies: {}
impl Arbitrary for Duration { # [inline] fn arbitrary (g : & mut Gen) -> Self { Self :: new_ranged (< _ > :: arbitrary (g) , < _ > :: arbitrary (g)) } # [inline] fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { Box :: new ((self . subsec_nanoseconds_ranged () , self . whole_seconds ()) . shrink () . map (| (mut nanoseconds , seconds) | { if (seconds > 0 && nanoseconds . get () < 0) || (seconds < 0 && nanoseconds . get () > 0) { nanoseconds = nanoseconds . neg () ; } Self :: new_ranged_unchecked (seconds , nanoseconds) }) ,) } }
};
}
