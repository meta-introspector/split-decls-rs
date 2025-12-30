// Generated macro for custom_coerce_unsize_info (function)
macro_rules! Depcratecustom_coerce_unsize_info {
() => {
// Module: crate
// Provides: {"custom_coerce_unsize_info"}
// Dependencies: {}
fn custom_coerce_unsize_info < 'tcx > (tcx : TyCtxtAt < 'tcx > , source_ty : Ty < 'tcx > , target_ty : Ty < 'tcx > ,) -> Result < CustomCoerceUnsized , ErrorGuaranteed > { let trait_ref = ty :: TraitRef :: new (tcx . tcx , tcx . require_lang_item (LangItem :: CoerceUnsized , tcx . span) , [source_ty , target_ty] ,) ; match tcx . codegen_select_candidate (ty :: TypingEnv :: fully_monomorphized () . as_query_input (trait_ref)) { Ok (traits :: ImplSource :: UserDefined (traits :: ImplSourceUserDefinedData { impl_def_id , .. })) => Ok (tcx . coerce_unsized_info (impl_def_id) ? . custom_kind . unwrap ()) , impl_source => { bug ! ("invalid `CoerceUnsized` from {source_ty} to {target_ty}: impl_source: {:?}" , impl_source) ; } } }
};
}
