macro_rules! symbol_name_provider {
    () => {
        fn symbol_name_provider < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx >) -> ty :: SymbolName < 'tcx > { let symbol_name = compute_symbol_name (tcx , instance , | | { if is_generic (instance) { instance . upstream_monomorphization (tcx) . unwrap_or (LOCAL_CRATE) } else { LOCAL_CRATE } }) ; ty :: SymbolName :: new (tcx , & symbol_name) }
    };
}

symbol_name_provider!()