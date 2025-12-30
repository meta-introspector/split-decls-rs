// Generated macro for EnumVariantSameName (struct)
macro_rules! Depcrate_errorsEnumVariantSameName {
() => {
// Module: crate::errors
// Provides: {"EnumVariantSameName"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [note (passes_enum_variant_same_name)] pub (crate) struct EnumVariantSameName < 'tcx > { # [primary_span] pub variant_span : Span , pub dead_name : Symbol , pub dead_descr : & 'tcx str , }
};
}
