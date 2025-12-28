macro_rules! SymbolAlreadyDefined {
    () => {
        # [derive (Diagnostic)] # [diag (monomorphize_symbol_already_defined)] pub (crate) struct SymbolAlreadyDefined { # [primary_span] pub span : Option < Span > , pub symbol : String , }
    };
}

SymbolAlreadyDefined!();