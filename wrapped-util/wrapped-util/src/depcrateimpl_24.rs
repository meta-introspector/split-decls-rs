// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl FromStrRadix for i32 { fn from_str_radix (s : & str , radix : u32) -> Result < Self , ParseIntError > { let s = strip_radix_prefix (s , radix) ; i32 :: from_str_radix (s , radix) } }
};
}
