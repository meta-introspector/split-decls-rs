// Generated macro for remove_bom (function)
macro_rules! Depcrateremove_bom {
() => {
// Module: crate
// Provides: {"remove_bom"}
// Dependencies: {}
# [doc = " Removes UTF-8 BOM, if any."] fn remove_bom (src : & mut String , normalized_pos : & mut Vec < NormalizedPos >) { if src . starts_with ('\u{feff}') { src . drain (.. 3) ; normalized_pos . push (NormalizedPos { pos : RelativeBytePos (0) , diff : 3 }) ; } }
};
}
