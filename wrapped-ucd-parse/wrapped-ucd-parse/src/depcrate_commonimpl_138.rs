// Generated macro for impl_138 (impl)
macro_rules! Depcrate_commonimpl_138 {
() => {
// Module: crate::common
// Provides: {"impl_138"}
// Dependencies: {}
impl FromStr for Codepoint { type Err = Error ; fn from_str (s : & str) -> Result < Codepoint , Error > { match u32 :: from_str_radix (s , 16) { Ok (n) => Codepoint :: from_u32 (n) , Err (err) => { return err ! ("failed to parse '{}' as a hexadecimal codepoint: {}" , s , err) ; } } } }
};
}
