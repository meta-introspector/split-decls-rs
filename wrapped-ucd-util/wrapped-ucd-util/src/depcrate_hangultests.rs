// Generated macro for tests (module)
macro_rules! Depcrate_hangultests {
() => {
// Module: crate::hangul
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: unicode_tables :: jamo_short_name :: JAMO_SHORT_NAME as TABLE ; use super :: { hangul_full_canonical_decomposition , hangul_name } ; # [test] fn canon_decomp () { assert_eq ! (hangul_full_canonical_decomposition (0xD4DB) , Some ((0x1111 , 0x1171 , Some (0x11B6)))) ; } # [test] fn name () { assert_eq ! (hangul_name (TABLE , 0xD4DB) . unwrap () , "HANGUL SYLLABLE PWILH") ; } # [test] fn all () { for cp in 0xAC00 .. (0xD7A3 + 1) { hangul_name (TABLE , cp) . unwrap () ; } } # [test] fn invalid () { assert ! (hangul_name (TABLE , 0) . is_none ()) ; } }
};
}
