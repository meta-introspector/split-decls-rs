// Generated macro for find_doc (function)
macro_rules! Depcratefind_doc {
() => {
// Module: crate
// Provides: {"find_doc"}
// Dependencies: {}
fn find_doc (attrs : & [syn :: Attribute]) -> String { attrs . iter () . filter_map (| a | { # [allow (clippy :: collapsible_if)] if let syn :: Meta :: NameValue (ref l) = a . meta { if l . path . is_ident ("doc") { if let syn :: Expr :: Lit (syn :: ExprLit { lit : syn :: Lit :: Str (ref s) , .. }) = l . value { return Some (s . value ()) ; } } } None }) . collect () }
};
}
