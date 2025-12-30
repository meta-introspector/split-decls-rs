// Generated macro for AttrApplication (enum)
macro_rules! Depcrate_errorsAttrApplication {
() => {
// Module: crate::errors
// Provides: {"AttrApplication"}
// Dependencies: {}
# [derive (Diagnostic)] pub (crate) enum AttrApplication { # [diag (passes_attr_application_enum , code = E0517)] Enum { # [primary_span] hint_span : Span , # [label] span : Span , } , # [diag (passes_attr_application_struct , code = E0517)] Struct { # [primary_span] hint_span : Span , # [label] span : Span , } , # [diag (passes_attr_application_struct_union , code = E0517)] StructUnion { # [primary_span] hint_span : Span , # [label] span : Span , } , # [diag (passes_attr_application_struct_enum_union , code = E0517)] StructEnumUnion { # [primary_span] hint_span : Span , # [label] span : Span , } , }
};
}
