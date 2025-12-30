// Generated macro for assert_symbols_are_distinct (function)
macro_rules! Depcrate_partitioningassert_symbols_are_distinct {
() => {
// Module: crate::partitioning
// Provides: {"assert_symbols_are_distinct"}
// Dependencies: {}
# [inline (never)] fn assert_symbols_are_distinct < 'a , 'tcx , I > (tcx : TyCtxt < 'tcx > , mono_items : I) where I : Iterator < Item = & 'a MonoItem < 'tcx > > , 'tcx : 'a , { let _prof_timer = tcx . prof . generic_activity ("assert_symbols_are_distinct") ; let mut symbols : Vec < _ > = mono_items . map (| mono_item | (mono_item , mono_item . symbol_name (tcx))) . collect () ; symbols . sort_by_key (| sym | sym . 1) ; for & [(mono_item1 , ref sym1) , (mono_item2 , ref sym2)] in symbols . array_windows () { if sym1 == sym2 { let span1 = mono_item1 . local_span (tcx) ; let span2 = mono_item2 . local_span (tcx) ; let span = match (span1 , span2) { (Some (span1) , Some (span2)) => { Some (if span1 . lo () . 0 > span2 . lo () . 0 { span1 } else { span2 }) } (span1 , span2) => span1 . or (span2) , } ; tcx . dcx () . emit_fatal (SymbolAlreadyDefined { span , symbol : sym1 . to_string () }) ; } } }
};
}
