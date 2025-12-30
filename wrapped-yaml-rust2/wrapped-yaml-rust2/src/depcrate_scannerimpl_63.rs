// Generated macro for impl_63 (impl)
macro_rules! Depcrate_scannerimpl_63 {
() => {
// Module: crate::scanner
// Provides: {"impl_63"}
// Dependencies: {}
impl Marker { fn new (index : usize , line : usize , col : usize) -> Marker { Marker { index , line , col } } # [doc = " Return the index (in bytes) of the marker in the source."] # [must_use] pub fn index (& self) -> usize { self . index } # [doc = " Return the line of the marker in the source."] # [must_use] pub fn line (& self) -> usize { self . line } # [doc = " Return the column of the marker in the source."] # [must_use] pub fn col (& self) -> usize { self . col } }
};
}
