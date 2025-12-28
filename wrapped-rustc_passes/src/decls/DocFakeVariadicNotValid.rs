macro_rules! DocFakeVariadicNotValid {
    () => {
        # [derive (Diagnostic)] # [diag (passes_doc_fake_variadic_not_valid)] pub (crate) struct DocFakeVariadicNotValid { # [primary_span] pub span : Span , }
    };
}

DocFakeVariadicNotValid!()