// Generated macro for tests (module)
macro_rules! Depcrate_grapheme_cluster_breaktests {
() => {
// Module: crate::grapheme_cluster_break
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { GraphemeClusterBreak , GraphemeClusterBreakTest } ; # [test] fn parse_single () { let line = "093B          ; SpacingMark # Mc       DEVANAGARI VOWEL SIGN OOE\n" ; let row : GraphemeClusterBreak = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x093B) ; assert_eq ! (row . value , "SpacingMark") ; } # [test] fn parse_range () { let line = "1F1E6..1F1FF  ; Regional_Indicator # So  [26] REGIONAL INDICATOR SYMBOL LETTER A..REGIONAL INDICATOR SYMBOL LETTER Z\n" ; let row : GraphemeClusterBreak = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x1F1E6 , 0x1F1FF)) ; assert_eq ! (row . value , "Regional_Indicator") ; } # [test] fn parse_test () { let line = "÷ 0061 × 1F3FF ÷ 1F476 × 200D × 1F6D1 ÷	#  ÷ [0.2] LATIN SMALL LETTER A (Other) × [9.0] EMOJI MODIFIER FITZPATRICK TYPE-6 (Extend) ÷ [999.0] BABY (ExtPict) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [11.0] OCTAGONAL SIGN (ExtPict) ÷ [0.3]\n" ; let row : GraphemeClusterBreakTest = line . parse () . unwrap () ; assert_eq ! (row . grapheme_clusters , vec ! ["\u{0061}\u{1F3FF}" , "\u{1F476}\u{200D}\u{1F6D1}" ,]) ; assert ! (row . comment . starts_with ("÷ [0.2] LATIN SMALL LETTER A")) ; } }
};
}
