macro_rules! UnsupportedAttributesInWhere {
    () => {
        # [derive (Diagnostic)] # [diag (passes_unsupported_attributes_in_where)] # [help] pub (crate) struct UnsupportedAttributesInWhere { # [primary_span] pub span : MultiSpan , }
    };
}

UnsupportedAttributesInWhere!();