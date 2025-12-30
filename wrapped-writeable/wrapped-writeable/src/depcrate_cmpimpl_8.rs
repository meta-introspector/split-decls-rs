// Generated macro for impl_8 (impl)
macro_rules! Depcrate_cmpimpl_8 {
() => {
// Module: crate::cmp
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'a > WriteComparator < 'a > { # [inline] fn new (code_units : & 'a [u8]) -> Self { Self { code_units , result : Ordering :: Equal , } } # [inline] fn finish (self) -> Ordering { if matches ! (self . result , Ordering :: Equal) && ! self . code_units . is_empty () { Ordering :: Greater } else { self . result } } }
};
}
