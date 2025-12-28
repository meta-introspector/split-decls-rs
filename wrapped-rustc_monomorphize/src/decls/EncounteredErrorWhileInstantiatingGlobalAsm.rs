macro_rules! EncounteredErrorWhileInstantiatingGlobalAsm {
    () => {
        # [derive (Diagnostic)] # [diag (monomorphize_encountered_error_while_instantiating_global_asm)] pub (crate) struct EncounteredErrorWhileInstantiatingGlobalAsm { # [primary_span] pub span : Span , }
    };
}

EncounteredErrorWhileInstantiatingGlobalAsm!()