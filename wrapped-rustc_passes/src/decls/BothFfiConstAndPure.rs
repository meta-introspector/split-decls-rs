macro_rules! BothFfiConstAndPure {
    () => {
        # [derive (Diagnostic)] # [diag (passes_both_ffi_const_and_pure , code = E0757)] pub (crate) struct BothFfiConstAndPure { # [primary_span] pub attr_span : Span , }
    };
}

BothFfiConstAndPure!()