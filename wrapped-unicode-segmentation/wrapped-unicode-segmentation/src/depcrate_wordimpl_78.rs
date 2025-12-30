// Generated macro for impl_78 (impl)
macro_rules! Depcrate_wordimpl_78 {
() => {
// Module: crate::word
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'a > AsciiWordBoundIter < 'a > { pub fn new (s : & 'a str) -> Self { AsciiWordBoundIter { rest : s , offset : 0 } } # [inline] fn is_core (b : u8) -> bool { b . is_ascii_alphanumeric () || b == b'_' } # [inline] fn is_infix (b : u8 , prev : u8 , next : u8) -> bool { match b { b'.' | b',' | b';' | b'\'' if prev . is_ascii_digit () && next . is_ascii_digit () => true , b'\'' | b'.' | b':' if prev . is_ascii_alphabetic () && next . is_ascii_alphabetic () => true , _ => false , } } }
};
}
