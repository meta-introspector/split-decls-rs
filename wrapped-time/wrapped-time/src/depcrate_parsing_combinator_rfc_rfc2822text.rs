// Generated macro for text (function)
macro_rules! Depcrate_parsing_combinator_rfc_rfc2822text {
() => {
// Module: crate::parsing::combinator::rfc::rfc2822
// Provides: {"text"}
// Dependencies: {}
# [doc = " Consume the `text` rule."] # [inline] fn text < 'a > (input : & 'a [u8]) -> ParsedItem < 'a , () > { let new_text = | input : & 'a [u8] | match input { [1 ..= 9 | 11 ..= 12 | 14 ..= 127 , rest @ ..] => Some (ParsedItem (rest , ())) , _ => None , } ; let obs_char = | input : & 'a [u8] | match input { [b')' , ..] => None , [0 ..= 9 | 11 ..= 12 | 14 ..= 127 , rest @ ..] => Some (rest) , _ => None , } ; let obs_text = | mut input | { input = zero_or_more (ascii_char :: < b'\n' >) (input) . into_inner () ; input = zero_or_more (ascii_char :: < b'\r' >) (input) . into_inner () ; while let Some (rest) = obs_char (input) { input = rest ; input = zero_or_more (ascii_char :: < b'\n' >) (input) . into_inner () ; input = zero_or_more (ascii_char :: < b'\r' >) (input) . into_inner () ; } ParsedItem (input , ()) } ; new_text (input) . unwrap_or_else (| | obs_text (input)) }
};
}
