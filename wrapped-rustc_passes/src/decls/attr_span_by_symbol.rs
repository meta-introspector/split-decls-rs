macro_rules! deps {
    () => {
        EntryContext!();
    };
}

macro_rules! attr_span_by_symbol {
    () => {
        deps!();
        fn attr_span_by_symbol (ctxt : & EntryContext < '_ > , id : ItemId , sym : Symbol) -> Option < Span > { let attrs = ctxt . tcx . hir_attrs (id . hir_id ()) ; attr :: find_by_name (attrs , sym) . map (| attr | attr . span ()) }
    };
}

attr_span_by_symbol!();