// Generated macro for create_fn_mono_item (function)
macro_rules! Depcrate_collectorcreate_fn_mono_item {
() => {
// Module: crate::collector
// Provides: {"create_fn_mono_item"}
// Dependencies: {}
# [instrument (skip (tcx) , level = "debug" , ret)] fn create_fn_mono_item < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , source : Span ,) -> Spanned < MonoItem < 'tcx > > { let def_id = instance . def_id () ; if tcx . sess . opts . unstable_opts . profile_closures && def_id . is_local () && tcx . is_closure_like (def_id) { crate :: util :: dump_closure_profile (tcx , instance) ; } respan (source , MonoItem :: Fn (instance)) }
};
}
