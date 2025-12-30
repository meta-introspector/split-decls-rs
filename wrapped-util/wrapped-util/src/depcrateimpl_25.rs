// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
# [cfg (f16_enabled)] impl FromStrRadix for f16 { fn from_str_radix (s : & str , radix : u32) -> Result < Self , ParseIntError > { if radix == 16 && s . contains ("p") { return Ok (libm :: support :: hf16 (s)) ; } let s = strip_radix_prefix (s , radix) ; u16 :: from_str_radix (s , radix) . map (Self :: from_bits) } }
};
}
