// Generated macro for as_hex (function)
macro_rules! Depcrate_char_traitsas_hex {
() => {
// Module: crate::char_traits
// Provides: {"as_hex"}
// Dependencies: {}
# [doc = " Convert the hexadecimal digit to an integer."] # [inline] pub (crate) fn as_hex (c : char) -> u32 { match c { '0' ..= '9' => (c as u32) - ('0' as u32) , 'a' ..= 'f' => (c as u32) - ('a' as u32) + 10 , 'A' ..= 'F' => (c as u32) - ('A' as u32) + 10 , _ => unreachable ! () , } }
};
}
