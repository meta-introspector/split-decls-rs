// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl FromStrRadix for f32 { fn from_str_radix (s : & str , radix : u32) -> Result < Self , ParseIntError > { if radix == 16 && s . contains ("p") { return Ok (hf32 (s)) ; } let s = strip_radix_prefix (s , radix) ; u32 :: from_str_radix (s , radix) . map (Self :: from_bits) } }
};
}
