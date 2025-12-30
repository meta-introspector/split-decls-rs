// Generated macro for FieldIsPrivate (struct)
macro_rules! Depcrate_errorsFieldIsPrivate {
() => {
// Module: crate::errors
// Provides: {"FieldIsPrivate"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (privacy_field_is_private , code = E0451)] pub (crate) struct FieldIsPrivate { # [primary_span] pub span : MultiSpan , # [label] pub struct_span : Option < Span > , pub field_names : String , pub variant_descr : & 'static str , pub def_path_str : String , # [subdiagnostic] pub labels : Vec < FieldIsPrivateLabel > , pub len : usize , }
};
}
