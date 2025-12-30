// Generated macro for unwrap_fn_abi (function)
macro_rules! Depcrate_abi_testunwrap_fn_abi {
() => {
// Module: crate::abi_test
// Provides: {"unwrap_fn_abi"}
// Dependencies: {}
fn unwrap_fn_abi < 'tcx > (abi : Result < & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , & 'tcx FnAbiError < 'tcx > > , tcx : TyCtxt < 'tcx > , item_def_id : LocalDefId ,) -> & 'tcx FnAbi < 'tcx , Ty < 'tcx > > { match abi { Ok (abi) => abi , Err (FnAbiError :: Layout (layout_error)) => { tcx . dcx () . emit_fatal (Spanned { node : layout_error . into_diagnostic () , span : tcx . def_span (item_def_id) , }) ; } } }
};
}
