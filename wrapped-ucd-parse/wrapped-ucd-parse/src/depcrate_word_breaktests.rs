// Generated macro for tests (module)
macro_rules! Depcrate_word_breaktests {
() => {
// Module: crate::word_break
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { WordBreak , WordBreakTest } ; # [test] fn parse_single () { let line = "0A83          ; Extend # Mc       GUJARATI SIGN VISARGA\n" ; let row : WordBreak = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x0A83) ; assert_eq ! (row . value , "Extend") ; } # [test] fn parse_range () { let line = "104A0..104A9  ; Numeric # Nd  [10] OSMANYA DIGIT ZERO..OSMANYA DIGIT NINE\n" ; let row : WordBreak = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x104A0 , 0x104A9)) ; assert_eq ! (row . value , "Numeric") ; } # [test] fn parse_test () { let line = "÷ 0031 ÷ 0027 × 0308 ÷ 0061 ÷ 0027 × 2060 ÷	#  ÷ [0.2] DIGIT ONE (Numeric) ÷ [999.0] APOSTROPHE (Single_Quote) × [4.0] COMBINING DIAERESIS (Extend_FE) ÷ [999.0] LATIN SMALL LETTER A (ALetter) ÷ [999.0] APOSTROPHE (Single_Quote) × [4.0] WORD JOINER (Format_FE) ÷ [0.3]" ; let row : WordBreakTest = line . parse () . unwrap () ; assert_eq ! (row . words , vec ! ["\u{0031}" , "\u{0027}\u{0308}" , "\u{0061}" , "\u{0027}\u{2060}" ,]) ; assert ! (row . comment . contains ("[4.0] COMBINING DIAERESIS (Extend_FE)")) ; } }
};
}
