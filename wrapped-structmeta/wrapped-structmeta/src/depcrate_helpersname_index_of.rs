// Generated macro for name_index_of (function)
macro_rules! Depcrate_helpersname_index_of {
() => {
// Module: crate::helpers
// Provides: {"name_index_of"}
// Dependencies: {}
fn name_index_of (names : & [& str] , rest : bool , ident : & Ident ,) -> Option < std :: result :: Result < usize , Ident > > { if let Some (index) = find (names , ident) { Some (Ok (index)) } else if rest { Some (Err (ident . clone ())) } else { None } }
};
}
