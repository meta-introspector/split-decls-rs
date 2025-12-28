macro_rules! MissingConstErr {
    () => {
        # [derive (Diagnostic)] # [diag (passes_missing_const_err)] pub (crate) struct MissingConstErr { # [primary_span] # [help] pub fn_sig_span : Span , }
    };
}

MissingConstErr!()