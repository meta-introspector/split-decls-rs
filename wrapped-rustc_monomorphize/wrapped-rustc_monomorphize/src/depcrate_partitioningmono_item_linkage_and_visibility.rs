// Generated macro for mono_item_linkage_and_visibility (function)
macro_rules! Depcrate_partitioningmono_item_linkage_and_visibility {
() => {
// Module: crate::partitioning
// Provides: {"mono_item_linkage_and_visibility"}
// Dependencies: {}
fn mono_item_linkage_and_visibility < 'tcx > (tcx : TyCtxt < 'tcx > , mono_item : & MonoItem < 'tcx > , can_be_internalized : & mut bool , can_export_generics : bool , always_export_generics : bool ,) -> (Linkage , Visibility) { if let Some (explicit_linkage) = mono_item . explicit_linkage (tcx) { return (explicit_linkage , Visibility :: Default) ; } let vis = mono_item_visibility (tcx , mono_item , can_be_internalized , can_export_generics , always_export_generics ,) ; (Linkage :: External , vis) }
};
}
