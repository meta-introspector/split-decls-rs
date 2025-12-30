// Generated macro for impl_860 (impl)
macro_rules! Depcrate_quickcheckimpl_860 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_860"}
// Dependencies: {}
impl Arbitrary for Weekday { # [inline] fn arbitrary (g : & mut Gen) -> Self { use Weekday :: * ; match arbitrary_between ! (u8 ; g , 0 , 6) { 0 => Monday , 1 => Tuesday , 2 => Wednesday , 3 => Thursday , 4 => Friday , 5 => Saturday , val => { debug_assert ! (val == 6) ; Sunday } } } # [inline] fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { match self { Self :: Monday => empty_shrinker () , _ => single_shrinker (self . previous ()) , } } }
};
}
