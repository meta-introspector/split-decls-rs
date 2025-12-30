// Generated macro for impl_136 (impl)
macro_rules! Depcrate_commonimpl_136 {
() => {
// Module: crate::common
// Provides: {"impl_136"}
// Dependencies: {}
impl Codepoint { # [doc = " Create a new codepoint from a `u32`."] # [doc = ""] # [doc = " If the given number is not a valid codepoint, then this returns an"] # [doc = " error."] pub fn from_u32 (n : u32) -> Result < Codepoint , Error > { if n > 0x10FFFF { err ! ("{:x} is not a valid Unicode codepoint" , n) } else { Ok (Codepoint (n)) } } # [doc = " Return the underlying `u32` codepoint value."] pub fn value (self) -> u32 { self . 0 } # [doc = " Attempt to convert this codepoint to a Unicode scalar value."] # [doc = ""] # [doc = " If this is a surrogate codepoint, then this returns `None`."] pub fn scalar (self) -> Option < char > { char :: from_u32 (self . 0) } }
};
}
