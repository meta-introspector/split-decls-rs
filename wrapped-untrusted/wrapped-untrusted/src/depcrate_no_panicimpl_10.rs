// Generated macro for impl_10 (impl)
macro_rules! Depcrate_no_panicimpl_10 {
() => {
// Module: crate::no_panic
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a > Slice < 'a > { # [inline] pub const fn new (bytes : & 'a [u8]) -> Self { Self { bytes } } # [inline] pub fn get (& self , i : usize) -> Option < & u8 > { self . bytes . get (i) } # [inline] pub fn subslice (& self , r : core :: ops :: Range < usize >) -> Option < Self > { self . bytes . get (r) . map (| bytes | Self { bytes }) } # [inline] pub fn is_empty (& self) -> bool { self . bytes . is_empty () } # [inline] pub fn len (& self) -> usize { self . bytes . len () } # [inline] pub fn as_slice_less_safe (& self) -> & 'a [u8] { self . bytes } }
};
}
