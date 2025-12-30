// Generated macro for ideograph_name (function)
macro_rules! Depcrate_ideographideograph_name {
() => {
// Module: crate::ideograph
// Provides: {"ideograph_name"}
// Dependencies: {}
# [doc = " Return the character name of the given ideograph codepoint."] # [doc = ""] # [doc = " This operation is only defined on ideographic codepoints. This includes"] # [doc = " precisely the following inclusive ranges:"] # [doc = ""] # [doc = " * `3400..4DB5`"] # [doc = " * `4E00..9FD5`"] # [doc = " * `20000..2A6D6`"] # [doc = " * `2A700..2B734`"] # [doc = " * `2B740..2B81D`"] # [doc = " * `2B820..2CEA1`"] # [doc = " * `17000..187EC`"] # [doc = " * `F900..FA6D`"] # [doc = " * `FA70..FAD9`"] # [doc = " * `2F800..2FA1D`"] # [doc = ""] # [doc = " If the given codepoint is not in any of the above ranges, then `None` is"] # [doc = " returned."] # [doc = ""] # [doc = " This implements the algorithm described in Unicode 4.8."] pub fn ideograph_name (cp : u32) -> Option < String > { match cp { 0x3400 ..= 0x4DB5 | 0x4E00 ..= 0x9FD5 | 0x20000 ..= 0x2A6D6 | 0x2A700 ..= 0x2B734 | 0x2B740 ..= 0x2B81D | 0x2B820 ..= 0x2CEA1 => { Some (format ! ("CJK UNIFIED IDEOGRAPH-{:04X}" , cp)) } 0x17000 ..= 0x187EC => Some (format ! ("TANGUT IDEOGRAPH-{:04X}" , cp)) , 0xF900 ..= 0xFA6D | 0xFA70 ..= 0xFAD9 | 0x2F800 ..= 0x2FA1D => { Some (format ! ("CJK COMPATIBILITY IDEOGRAPH-{:04X}" , cp)) } _ => None , } }
};
}
