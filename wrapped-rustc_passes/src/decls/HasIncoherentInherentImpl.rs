macro_rules! HasIncoherentInherentImpl {
    () => {
        # [derive (Diagnostic)] # [diag (passes_has_incoherent_inherent_impl)] pub (crate) struct HasIncoherentInherentImpl { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }
    };
}

HasIncoherentInherentImpl!()