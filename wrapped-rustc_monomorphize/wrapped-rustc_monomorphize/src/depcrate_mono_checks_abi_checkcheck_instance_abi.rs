// Generated macro for check_instance_abi (function)
macro_rules! Depcrate_mono_checks_abi_checkcheck_instance_abi {
() => {
// Module: crate::mono_checks::abi_check
// Provides: {"check_instance_abi"}
// Dependencies: {}
# [doc = " Checks that the ABI of a given instance of a function does not contain vector-passed arguments"] # [doc = " or return values for which the corresponding target feature is not enabled."] fn check_instance_abi < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx >) { let typing_env = ty :: TypingEnv :: fully_monomorphized () ; let Ok (abi) = tcx . fn_abi_of_instance (typing_env . as_query_input ((instance , ty :: List :: empty ()))) else { tcx . dcx () . delayed_bug ("ABI computation failure should lead to compilation failure") ; return ; } ; let loc = | | { let def_id = instance . def_id () ; (tcx . def_span (def_id) , def_id . as_local () . map (| did | tcx . local_def_id_to_hir_id (did)) . unwrap_or (CRATE_HIR_ID) ,) } ; do_check_simd_vector_abi (tcx , abi , instance . def_id () , false , loc) ; }
};
}
