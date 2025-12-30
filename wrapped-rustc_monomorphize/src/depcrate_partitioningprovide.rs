// Generated macro for provide (function)
macro_rules! Depcrate_partitioningprovide {
() => {
// Module: crate::partitioning
// Provides: {"provide"}
// Dependencies: {}
pub (crate) fn provide (providers : & mut Providers) { providers . collect_and_partition_mono_items = collect_and_partition_mono_items ; providers . is_codegened_item = | tcx , def_id | tcx . collect_and_partition_mono_items (()) . all_mono_items . contains (& def_id) ; providers . codegen_unit = | tcx , name | { tcx . collect_and_partition_mono_items (()) . codegen_units . iter () . find (| cgu | cgu . name () == name) . unwrap_or_else (| | panic ! ("failed to find cgu with name {name:?}")) } ; providers . size_estimate = | tcx , instance | { match instance . def { InstanceKind :: Item (..) | InstanceKind :: DropGlue (..) | InstanceKind :: AsyncDropGlueCtorShim (..) => { let mir = tcx . instance_mir (instance . def) ; mir . basic_blocks . iter () . map (| bb | bb . statements . len () + 1) . sum () } _ => 1 , } } ; collector :: provide (providers) ; }
};
}
