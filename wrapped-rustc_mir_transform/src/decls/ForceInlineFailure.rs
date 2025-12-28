macro_rules! deps {
    () => {
        ForceInlineJustification!();
    };
}

macro_rules! ForceInlineFailure {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_transform_force_inline)] # [note] pub (crate) struct ForceInlineFailure { # [label (mir_transform_caller)] pub caller_span : Span , # [label (mir_transform_callee)] pub callee_span : Span , # [label (mir_transform_attr)] pub attr_span : Span , # [primary_span] # [label (mir_transform_call)] pub call_span : Span , pub callee : String , pub caller : String , pub reason : & 'static str , # [subdiagnostic] pub justification : Option < ForceInlineJustification > , }
    };
}

ForceInlineFailure!()