// Generated macro for tests (module)
macro_rules! Depcrate_line_breaktests {
() => {
// Module: crate::line_break
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: LineBreakTest ; # [test] fn parse_test () { let line = "× 1F1F7 × 1F1FA ÷ 1F1F8 × 1F1EA ÷   #  × [0.3] REGIONAL INDICATOR SYMBOL LETTER R (RI) × [30.11] REGIONAL INDICATOR SYMBOL LETTER U (RI) ÷ [30.13] REGIONAL INDICATOR SYMBOL LETTER S (RI) × [30.11] REGIONAL INDICATOR SYMBOL LETTER E (RI) ÷ [0.3]" ; let row : LineBreakTest = line . parse () . unwrap () ; assert_eq ! (row . lines , vec ! ["\u{1F1F7}\u{1F1FA}" , "\u{1F1F8}\u{1F1EA}" ,]) ; assert ! (row . comment . ends_with ("(RI) ÷ [0.3]")) ; } }
};
}
