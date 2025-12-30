// Generated macro for UnexportableItem (enum)
macro_rules! Depcrate_errorsUnexportableItem {
() => {
// Module: crate::errors
// Provides: {"UnexportableItem"}
// Dependencies: {}
# [derive (Diagnostic)] pub (crate) enum UnexportableItem < 'a > { # [diag (passes_unexportable_item)] Item { # [primary_span] span : Span , descr : & 'a str , } , # [diag (passes_unexportable_generic_fn)] GenericFn (# [primary_span] Span) , # [diag (passes_unexportable_fn_abi)] FnAbi (# [primary_span] Span) , # [diag (passes_unexportable_type_repr)] TypeRepr (# [primary_span] Span) , # [diag (passes_unexportable_type_in_interface)] TypeInInterface { # [primary_span] span : Span , desc : & 'a str , ty : & 'a str , # [label] ty_span : Span , } , # [diag (passes_unexportable_priv_item)] PrivItem { # [primary_span] span : Span , # [note] vis_note : Span , vis_descr : & 'a str , } , # [diag (passes_unexportable_adt_with_private_fields)] AdtWithPrivFields { # [primary_span] span : Span , # [note] vis_note : Span , field_name : & 'a str , } , }
};
}
