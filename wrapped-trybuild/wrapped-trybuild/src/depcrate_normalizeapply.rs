// Generated macro for apply (function)
macro_rules! Depcrate_normalizeapply {
() => {
// Module: crate::normalize
// Provides: {"apply"}
// Dependencies: {}
fn apply (original : & str , normalization : Normalization , context : Context) -> String { let mut normalized = String :: new () ; let lines : Vec < & str > = original . lines () . collect () ; let mut filter = Filter { all_lines : & lines , normalization , context , hide_numbers : 0 , other_types : None , } ; for i in 0 .. lines . len () { if let Some (line) = filter . apply (i) { normalized += & line ; if ! normalized . ends_with ("\n\n") { normalized . push ('\n') ; } } } normalized = unindent (normalized , normalization) ; trim (normalized) }
};
}
