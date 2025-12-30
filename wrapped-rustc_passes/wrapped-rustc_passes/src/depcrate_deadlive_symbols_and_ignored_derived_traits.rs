// Generated macro for live_symbols_and_ignored_derived_traits (function)
macro_rules! Depcrate_deadlive_symbols_and_ignored_derived_traits {
() => {
// Module: crate::dead
// Provides: {"live_symbols_and_ignored_derived_traits"}
// Dependencies: {}
fn live_symbols_and_ignored_derived_traits (tcx : TyCtxt < '_ > , () : () ,) -> (LocalDefIdSet , LocalDefIdMap < FxIndexSet < DefId > >) { let (worklist , mut unsolved_items) = create_and_seed_worklist (tcx) ; let mut symbol_visitor = MarkSymbolVisitor { worklist , tcx , maybe_typeck_results : None , scanned : Default :: default () , live_symbols : Default :: default () , repr_unconditionally_treats_fields_as_live : false , repr_has_repr_simd : false , in_pat : false , ignore_variant_stack : vec ! [] , ignored_derived_traits : Default :: default () , } ; symbol_visitor . mark_live_symbols () ; let mut items_to_check : Vec < _ > = unsolved_items . extract_if (.. , | & mut local_def_id | { symbol_visitor . check_impl_or_impl_item_live (local_def_id) }) . collect () ; while ! items_to_check . is_empty () { symbol_visitor . worklist . extend (items_to_check . drain (..) . map (| id | (id , ComesFromAllowExpect :: No))) ; symbol_visitor . mark_live_symbols () ; items_to_check . extend (unsolved_items . extract_if (.. , | & mut local_def_id | { symbol_visitor . check_impl_or_impl_item_live (local_def_id) })) ; } (symbol_visitor . live_symbols , symbol_visitor . ignored_derived_traits) }
};
}
