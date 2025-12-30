// Generated macro for type_for_field_schema (function)
macro_rules! Depcrate_schema_exprstype_for_field_schema {
() => {
// Module: crate::schema_exprs
// Provides: {"type_for_field_schema"}
// Dependencies: {}
fn type_for_field_schema (cont : & Container , field : & Field) -> (syn :: Type , Option < TokenStream >) { match & field . attrs . with { None => (field . ty . clone () , None) , Some (with_attr) => type_for_schema (cont , with_attr) , } }
};
}
