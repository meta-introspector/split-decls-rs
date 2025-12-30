// Generated macro for impl_861 (impl)
macro_rules! Depcrate_quickcheckimpl_861 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_861"}
// Dependencies: {}
impl Arbitrary for Month { # [inline] fn arbitrary (g : & mut Gen) -> Self { use Month :: * ; match arbitrary_between ! (u8 ; g , 1 , 12) { 1 => January , 2 => February , 3 => March , 4 => April , 5 => May , 6 => June , 7 => July , 8 => August , 9 => September , 10 => October , 11 => November , val => { debug_assert ! (val == 12) ; December } } } # [inline] fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { match self { Self :: January => empty_shrinker () , _ => single_shrinker (self . previous ()) , } } }
};
}
