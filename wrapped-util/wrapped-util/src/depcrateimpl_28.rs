// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
# [cfg (f128_enabled)] impl FromStrRadix for f128 { fn from_str_radix (s : & str , radix : u32) -> Result < Self , ParseIntError > { if radix == 16 && s . contains ("p") { return Ok (libm :: support :: hf128 (s)) ; } let s = strip_radix_prefix (s , radix) ; u128 :: from_str_radix (s , radix) . map (Self :: from_bits) } }
};
}
