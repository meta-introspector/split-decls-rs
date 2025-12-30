// Generated macro for ItemIsPrivate (struct)
macro_rules! Depcrate_errorsItemIsPrivate {
() => {
// Module: crate::errors
// Provides: {"ItemIsPrivate"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (privacy_item_is_private)] pub (crate) struct ItemIsPrivate < 'a > { # [primary_span] # [label] pub span : Span , pub kind : & 'a str , pub descr : DiagArgFromDisplay < 'a > , }
};
}
