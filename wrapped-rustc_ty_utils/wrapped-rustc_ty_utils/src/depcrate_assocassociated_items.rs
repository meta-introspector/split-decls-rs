// Generated macro for associated_items (function)
macro_rules! Depcrate_assocassociated_items {
() => {
// Module: crate::assoc
// Provides: {"associated_items"}
// Dependencies: {}
fn associated_items (tcx : TyCtxt < '_ > , def_id : DefId) -> ty :: AssocItems { if tcx . is_trait_alias (def_id) { ty :: AssocItems :: new (Vec :: new ()) } else { let items = tcx . associated_item_def_ids (def_id) . iter () . map (| did | tcx . associated_item (* did)) ; ty :: AssocItems :: new (items) } }
};
}
