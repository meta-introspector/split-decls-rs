// Generated macro for fast_u16_to_str (function)
macro_rules! Depcrate_parserfast_u16_to_str {
() => {
// Module: crate::parser
// Provides: {"fast_u16_to_str"}
// Dependencies: {}
# [inline] fn fast_u16_to_str (buffer : & mut [u8 ; 5] , mut value : u16 ,) -> & str { let mut index = buffer . len () ; loop { index -= 1 ; buffer [index] = b'0' + (value % 10) as u8 ; value /= 10 ; if value == 0 { break ; } } unsafe { core :: str :: from_utf8_unchecked (& buffer [index ..]) } }
};
}
