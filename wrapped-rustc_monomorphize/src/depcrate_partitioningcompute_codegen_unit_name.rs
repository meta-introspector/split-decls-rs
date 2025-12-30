// Generated macro for compute_codegen_unit_name (function)
macro_rules! Depcrate_partitioningcompute_codegen_unit_name {
() => {
// Module: crate::partitioning
// Provides: {"compute_codegen_unit_name"}
// Dependencies: {}
fn compute_codegen_unit_name (tcx : TyCtxt < '_ > , name_builder : & mut CodegenUnitNameBuilder < '_ > , def_id : DefId , volatile : bool , cache : & mut CguNameCache ,) -> Symbol { let mut current_def_id = def_id ; let mut cgu_def_id = None ; loop { if current_def_id . is_crate_root () { if cgu_def_id . is_none () { cgu_def_id = Some (def_id . krate . as_def_id ()) ; } break ; } else if tcx . def_kind (current_def_id) == DefKind :: Mod { if cgu_def_id . is_none () { cgu_def_id = Some (current_def_id) ; } } else { cgu_def_id = None ; } current_def_id = tcx . parent (current_def_id) ; } let cgu_def_id = cgu_def_id . unwrap () ; * cache . entry ((cgu_def_id , volatile)) . or_insert_with (| | { let def_path = tcx . def_path (cgu_def_id) ; let components = def_path . data . iter () . map (| part | match part . data . name () { DefPathDataName :: Named (name) => name , DefPathDataName :: Anon { .. } => unreachable ! () , }) ; let volatile_suffix = volatile . then_some ("volatile") ; name_builder . build_cgu_name (def_path . krate , components , volatile_suffix) }) }
};
}
