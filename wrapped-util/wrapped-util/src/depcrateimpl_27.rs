// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl FromStrRadix for f64 { fn from_str_radix (s : & str , radix : u32) -> Result < Self , ParseIntError > { if s . contains ("p") { return Ok (hf64 (s)) ; } let s = strip_radix_prefix (s , radix) ; u64 :: from_str_radix (s , radix) . map (Self :: from_bits) } }
};
}
