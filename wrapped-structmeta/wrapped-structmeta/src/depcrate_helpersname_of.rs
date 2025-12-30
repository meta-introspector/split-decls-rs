// Generated macro for name_of (function)
macro_rules! Depcrate_helpersname_of {
() => {
// Module: crate::helpers
// Provides: {"name_of"}
// Dependencies: {}
fn name_of (names : & [& str] , rest : bool , ident : & Ident) -> Option < String > { if rest { Some (ident . to_string ()) } else { find (names , ident) . map (| i | names [i] . to_string ()) } }
};
}
