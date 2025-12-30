// Generated macro for check_recursion_limit (function)
macro_rules! Depcrate_collectorcheck_recursion_limit {
() => {
// Module: crate::collector
// Provides: {"check_recursion_limit"}
// Dependencies: {}
fn check_recursion_limit < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , span : Span , recursion_depths : & mut DefIdMap < usize > , recursion_limit : Limit ,) -> (DefId , usize) { let def_id = instance . def_id () ; let recursion_depth = recursion_depths . get (& def_id) . cloned () . unwrap_or (0) ; debug ! (" => recursion depth={}" , recursion_depth) ; let adjusted_recursion_depth = if tcx . is_lang_item (def_id , LangItem :: DropInPlace) { recursion_depth / 4 } else { recursion_depth } ; if ! recursion_limit . value_within_limit (adjusted_recursion_depth) { let def_span = tcx . def_span (def_id) ; let def_path_str = tcx . def_path_str (def_id) ; tcx . dcx () . emit_fatal (RecursionLimit { span , instance , def_span , def_path_str }) ; } recursion_depths . insert (def_id , recursion_depth + 1) ; (def_id , recursion_depth) }
};
}
