// Generated macro for loadable_from_disk (function)
macro_rules! Depcrate_plumbingloadable_from_disk {
() => {
// Module: crate::plumbing
// Provides: {"loadable_from_disk"}
// Dependencies: {}
pub (crate) fn loadable_from_disk < 'tcx > (tcx : TyCtxt < 'tcx > , id : SerializedDepNodeIndex) -> bool { if let Some (cache) = tcx . query_system . on_disk_cache . as_ref () { cache . loadable_from_disk (id) } else { false } }
};
}
