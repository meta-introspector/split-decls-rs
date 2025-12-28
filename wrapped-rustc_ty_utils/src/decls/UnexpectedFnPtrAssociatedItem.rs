macro_rules! UnexpectedFnPtrAssociatedItem {
    () => {
        # [derive (Diagnostic)] # [diag (ty_utils_unexpected_fnptr_associated_item)] pub (crate) struct UnexpectedFnPtrAssociatedItem { # [primary_span] pub span : Span , }
    };
}

UnexpectedFnPtrAssociatedItem!();