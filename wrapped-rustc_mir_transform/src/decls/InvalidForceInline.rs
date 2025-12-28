macro_rules! InvalidForceInline {
    () => {
        # [derive (Diagnostic)] # [diag (mir_transform_force_inline_attr)] # [note] pub (crate) struct InvalidForceInline { # [primary_span] pub attr_span : Span , # [label (mir_transform_callee)] pub callee_span : Span , pub callee : String , pub reason : & 'static str , }
    };
}

InvalidForceInline!()