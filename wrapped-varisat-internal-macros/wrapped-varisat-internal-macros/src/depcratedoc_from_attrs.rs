// Generated macro for doc_from_attrs (function)
macro_rules! Depcratedoc_from_attrs {
() => {
// Module: crate
// Provides: {"doc_from_attrs"}
// Dependencies: {}
# [doc = " Get the doc comment as LitStr from the attributes"] fn doc_from_attrs (attrs : & [Attribute]) -> Vec < LitStr > { let mut lines = vec ! [] ; for attr in attrs . iter () { if let Ok (Meta :: NameValue (MetaNameValue { path , lit : Lit :: Str (doc_str) , .. })) = attr . parse_meta () { if let Some (ident) = path . get_ident () { if ident == "doc" { lines . push (doc_str) ; } } } } lines }
};
}
