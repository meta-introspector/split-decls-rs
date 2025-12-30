// Generated macro for named_fields_static (function)
macro_rules! Depcrate_expandnamed_fields_static {
() => {
// Module: crate::expand
// Provides: {"named_fields_static"}
// Dependencies: {}
fn named_fields_static (name : & Ident , fields : & syn :: Fields , field_attrs : & [Attrs]) -> TokenStream { debug_assert ! (matches ! (fields , syn :: Fields :: Named (..))) ; let named_fields = fields . iter () . enumerate () . filter (| (i , _) | ! field_attrs [* i] . skip ()) . map (| (i , field) | { let field_name_literal = field_attrs [i] . rename (field . ident . as_ref () . unwrap ()) ; quote ! { :: valuable :: NamedField :: new (# field_name_literal) , } }) ; quote ! { static # name : & [:: valuable :: NamedField <'static >] = & [# (# named_fields) *] ; } }
};
}
