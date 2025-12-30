// Generated macro for is_camel_case (function)
macro_rules! Depcrate_nonstandard_styleis_camel_case {
() => {
// Module: crate::nonstandard_style
// Provides: {"is_camel_case"}
// Dependencies: {}
fn is_camel_case (name : & str) -> bool { let name = name . trim_matches ('_') ; if name . is_empty () { return true ; } ! name . chars () . next () . unwrap () . is_lowercase () && ! name . contains ("__") && ! name . chars () . collect :: < Vec < _ > > () . array_windows () . any (| & [fst , snd] | { char_has_case (fst) && snd == '_' || char_has_case (snd) && fst == '_' }) }
};
}
