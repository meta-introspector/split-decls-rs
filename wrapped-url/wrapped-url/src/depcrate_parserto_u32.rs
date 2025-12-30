// Generated macro for to_u32 (function)
macro_rules! Depcrate_parserto_u32 {
() => {
// Module: crate::parser
// Provides: {"to_u32"}
// Dependencies: {}
# [inline] pub fn to_u32 (i : usize) -> ParseResult < u32 > { if i <= u32 :: MAX as usize { Ok (i as u32) } else { Err (ParseError :: Overflow) } }
};
}
