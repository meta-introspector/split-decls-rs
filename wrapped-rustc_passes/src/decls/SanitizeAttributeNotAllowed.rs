macro_rules! SanitizeAttributeNotAllowed {
    () => {
        # [doc = " \"sanitize attribute not allowed here\""] # [derive (Diagnostic)] # [diag (passes_sanitize_attribute_not_allowed)] pub (crate) struct SanitizeAttributeNotAllowed { # [primary_span] pub attr_span : Span , # [doc = " \"not a function, impl block, or module\""] # [label (passes_not_fn_impl_mod)] pub not_fn_impl_mod : Option < Span > , # [doc = " \"function has no body\""] # [label (passes_no_body)] pub no_body : Option < Span > , # [doc = " \"sanitize attribute can be applied to a function (with body), impl block, or module\""] # [help] pub help : () , }
    };
}

SanitizeAttributeNotAllowed!()