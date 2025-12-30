// Generated macro for exportable_items_provider_local (function)
macro_rules! Depcrate_check_exportexportable_items_provider_local {
() => {
// Module: crate::check_export
// Provides: {"exportable_items_provider_local"}
// Dependencies: {}
# [doc = " Exportable items:"] # [doc = ""] # [doc = " 1. Structs/enums/unions with a stable representation (e.g. repr(i32) or repr(C))."] # [doc = " 2. Primitive types."] # [doc = " 3. Non-generic functions with a stable ABI (e.g. extern \"C\") for which every user"] # [doc = "    defined type used in the signature is also marked as `#[export]`."] fn exportable_items_provider_local < 'tcx > (tcx : TyCtxt < 'tcx > , _ : LocalCrate) -> & 'tcx [DefId] { if ! tcx . crate_types () . contains (& CrateType :: Sdylib) && ! tcx . is_sdylib_interface_build () { return & [] ; } let mut visitor = ExportableItemCollector :: new (tcx) ; tcx . hir_walk_toplevel_module (& mut visitor) ; let exportable_items = visitor . exportable_items ; for item_id in exportable_items . iter () { let mut validator = ExportableItemsChecker { tcx , exportable_items : & exportable_items , item_id : * item_id } ; validator . check () ; } tcx . arena . alloc_from_iter (exportable_items . into_iter ()) }
};
}
