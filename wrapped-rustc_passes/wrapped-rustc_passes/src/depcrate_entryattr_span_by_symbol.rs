// Generated macro for attr_span_by_symbol (function)
macro_rules! Depcrate_entryattr_span_by_symbol {
() => {
// Module: crate::entry
// Provides: {"attr_span_by_symbol"}
// Dependencies: {}
fn attr_span_by_symbol (ctxt : & EntryContext < '_ > , id : ItemId , sym : Symbol) -> Option < Span > { let attrs = ctxt . tcx . hir_attrs (id . hir_id ()) ; attr :: find_by_name (attrs , sym) . map (| attr | attr . span ()) }
};
}
