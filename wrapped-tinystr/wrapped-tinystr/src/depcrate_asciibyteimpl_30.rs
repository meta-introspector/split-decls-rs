// Generated macro for impl_30 (impl)
macro_rules! Depcrate_asciibyteimpl_30 {
() => {
// Module: crate::asciibyte
// Provides: {"impl_30"}
// Dependencies: {}
impl AsciiByte { # [doc = " Convert [u8; N] to [AsciiByte; N]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " All bytes MUST be in the range 0 to 127 inclusive."] # [inline] pub const unsafe fn to_ascii_byte_array < const N : usize > (bytes : & [u8 ; N]) -> [AsciiByte ; N] { * (bytes as * const [u8 ; N] as * const [AsciiByte ; N]) } # [inline] pub (crate) fn from_decimal_digit (digit : u8) -> AsciiByte { match digit { 0 => AsciiByte :: B48 , 1 => AsciiByte :: B49 , 2 => AsciiByte :: B50 , 3 => AsciiByte :: B51 , 4 => AsciiByte :: B52 , 5 => AsciiByte :: B53 , 6 => AsciiByte :: B54 , 7 => AsciiByte :: B55 , 8 => AsciiByte :: B56 , 9 => AsciiByte :: B57 , _ => { debug_assert ! (false , "not a single digit: {digit}") ; AsciiByte :: B32 } } } }
};
}
