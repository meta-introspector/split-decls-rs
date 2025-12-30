// Generated macro for add_features (function)
macro_rules! Depcrate_generatoradd_features {
() => {
// Module: crate::generator
// Provides: {"add_features"}
// Dependencies: {}
fn add_features (features : & mut BTreeSet < String > , ty : & impl TraverseType) { ty . traverse_type (& mut | ident | { let ident = ident . to_string () ; if ! BUILTIN_IDENTS . contains (ident . as_str ()) { features . insert (ident) ; } }) ; }
};
}
