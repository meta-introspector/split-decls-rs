// Generated macro for find_target_feature (function)
macro_rules! Depcratefind_target_feature {
() => {
// Module: crate
// Provides: {"find_target_feature"}
// Dependencies: {}
fn find_target_feature (attrs : & [syn :: Attribute]) -> Option < syn :: Lit > { attrs . iter () . flat_map (| a | { # [allow (clippy :: collapsible_if)] if let syn :: Meta :: List (ref l) = a . meta { if l . path . is_ident ("target_feature") { if let Ok (l) = syn :: punctuated :: Punctuated :: < syn :: Meta , Token ! [,] > :: parse_terminated . parse2 (l . tokens . clone ()) { return l ; } } } syn :: punctuated :: Punctuated :: new () }) . find_map (| m | match m { syn :: Meta :: NameValue (i) if i . path . is_ident ("enable") => { if let syn :: Expr :: Lit (lit) = i . value { Some (lit . lit) } else { None } } _ => None , }) }
};
}
