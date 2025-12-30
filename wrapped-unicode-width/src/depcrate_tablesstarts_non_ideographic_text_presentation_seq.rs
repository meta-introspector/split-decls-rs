// Generated macro for starts_non_ideographic_text_presentation_seq (function)
macro_rules! Depcrate_tablesstarts_non_ideographic_text_presentation_seq {
() => {
// Module: crate::tables
// Provides: {"starts_non_ideographic_text_presentation_seq"}
// Dependencies: {}
# [doc = " Returns `true` if `c` has default emoji presentation, but forms a [text presentation sequence]"] # [doc = " (https://www.unicode.org/reports/tr51/#def_text_presentation_sequence)"] # [doc = " when followed by `'\\u{FEOE}'`, and is not ideographic."] # [doc = " Such sequences are considered to have width 1."] # [inline] pub fn starts_non_ideographic_text_presentation_seq (c : char) -> bool { let cp : u32 = c . into () ; let top_bits = cp >> 8 ; let leaf : & [(u8 , u8)] = match top_bits { 0x23 => & TEXT_PRESENTATION_LEAF_0 , 0x25 => & TEXT_PRESENTATION_LEAF_1 , 0x26 => & TEXT_PRESENTATION_LEAF_2 , 0x27 => & TEXT_PRESENTATION_LEAF_3 , 0x2B => & TEXT_PRESENTATION_LEAF_4 , 0x1F0 => & TEXT_PRESENTATION_LEAF_5 , 0x1F3 => & TEXT_PRESENTATION_LEAF_6 , 0x1F4 => & TEXT_PRESENTATION_LEAF_7 , 0x1F5 => & TEXT_PRESENTATION_LEAF_8 , 0x1F6 => & TEXT_PRESENTATION_LEAF_9 , _ => return false , } ; let bottom_bits = (cp & 0xFF) as u8 ; leaf . binary_search_by (| & (lo , hi) | { if bottom_bits < lo { Ordering :: Greater } else if bottom_bits > hi { Ordering :: Less } else { Ordering :: Equal } }) . is_ok () }
};
}
