// Generated macro for tests (module)
macro_rules! Depcrate_sentence_breaktests {
() => {
// Module: crate::sentence_break
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { SentenceBreak , SentenceBreakTest } ; # [test] fn parse_single () { let line = "11445         ; Extend # Mc       NEWA SIGN VISARGA\n" ; let row : SentenceBreak = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x11445) ; assert_eq ! (row . value , "Extend") ; } # [test] fn parse_range () { let line = "FE31..FE32    ; SContinue # Pd   [2] PRESENTATION FORM FOR VERTICAL EM DASH..PRESENTATION FORM FOR VERTICAL EN DASH\n" ; let row : SentenceBreak = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0xFE31 , 0xFE32)) ; assert_eq ! (row . value , "SContinue") ; } # [test] fn parse_test () { let line = "÷ 2060 × 5B57 × 2060 × 002E × 2060 ÷ 5B57 × 2060 × 2060 ÷	#  ÷ [0.2] WORD JOINER (Format_FE) × [998.0] CJK UNIFIED IDEOGRAPH-5B57 (OLetter) × [5.0] WORD JOINER (Format_FE) × [998.0] FULL STOP (ATerm) × [5.0] WORD JOINER (Format_FE) ÷ [11.0] CJK UNIFIED IDEOGRAPH-5B57 (OLetter) × [5.0] WORD JOINER (Format_FE) × [5.0] WORD JOINER (Format_FE) ÷ [0.3]" ; let row : SentenceBreakTest = line . parse () . unwrap () ; assert_eq ! (row . sentences , vec ! ["\u{2060}\u{5B57}\u{2060}\u{002E}\u{2060}" , "\u{5B57}\u{2060}\u{2060}" ,]) ; assert ! (row . comment . contains ("[5.0] WORD JOINER (Format_FE)")) ; } }
};
}
