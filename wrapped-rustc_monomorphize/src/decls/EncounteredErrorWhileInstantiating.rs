macro_rules! EncounteredErrorWhileInstantiating {
    () => {
        # [derive (Diagnostic)] # [diag (monomorphize_encountered_error_while_instantiating)] pub (crate) struct EncounteredErrorWhileInstantiating < 'tcx > { # [primary_span] pub span : Span , pub kind : & 'static str , pub instance : Instance < 'tcx > , }
    };
}

EncounteredErrorWhileInstantiating!();