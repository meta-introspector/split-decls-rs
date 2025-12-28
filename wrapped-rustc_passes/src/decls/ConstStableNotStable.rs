macro_rules! ConstStableNotStable {
    () => {
        # [derive (Diagnostic)] # [diag (passes_const_stable_not_stable)] pub (crate) struct ConstStableNotStable { # [primary_span] pub fn_sig_span : Span , # [label] pub const_span : Span , }
    };
}

ConstStableNotStable!()