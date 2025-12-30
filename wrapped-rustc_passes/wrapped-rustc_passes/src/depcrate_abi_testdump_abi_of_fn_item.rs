// Generated macro for dump_abi_of_fn_item (function)
macro_rules! Depcrate_abi_testdump_abi_of_fn_item {
() => {
// Module: crate::abi_test
// Provides: {"dump_abi_of_fn_item"}
// Dependencies: {}
fn dump_abi_of_fn_item (tcx : TyCtxt < '_ > , item_def_id : LocalDefId , attr : & Attribute) { let typing_env = ty :: TypingEnv :: post_analysis (tcx , item_def_id) ; let args = GenericArgs :: identity_for_item (tcx , item_def_id) ; let instance = match Instance :: try_resolve (tcx , typing_env , item_def_id . into () , args) { Ok (Some (instance)) => instance , Ok (None) => { let ty = tcx . type_of (item_def_id) . instantiate_identity () ; tcx . dcx () . emit_fatal (Spanned { node : LayoutError :: Unknown (ty) . into_diagnostic () , span : tcx . def_span (item_def_id) , }) ; } Err (_guaranteed) => return , } ; let abi = unwrap_fn_abi (tcx . fn_abi_of_instance (typing_env . as_query_input ((instance , ty :: List :: empty ())) ,) , tcx , item_def_id ,) ; let meta_items = attr . meta_item_list () . unwrap_or_default () ; for meta_item in meta_items { match meta_item . name () { Some (sym :: debug) => { let fn_name = tcx . item_name (item_def_id) ; tcx . dcx () . emit_err (AbiOf { span : tcx . def_span (item_def_id) , fn_name , fn_abi : format ! ("{:#?}" , abi) , }) ; } _ => { tcx . dcx () . emit_err (UnrecognizedArgument { span : meta_item . span () }) ; } } } }
};
}
