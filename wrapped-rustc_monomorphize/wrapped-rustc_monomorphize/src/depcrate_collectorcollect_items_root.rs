// Generated macro for collect_items_root (function)
macro_rules! Depcrate_collectorcollect_items_root {
() => {
// Module: crate::collector
// Provides: {"collect_items_root"}
// Dependencies: {}
fn collect_items_root < 'tcx > (tcx : TyCtxt < 'tcx > , starting_item : Spanned < MonoItem < 'tcx > > , state : & SharedState < 'tcx > , recursion_limit : Limit ,) { if ! state . visited . lock_mut () . insert (starting_item . node) { return ; } let mut recursion_depths = DefIdMap :: default () ; collect_items_rec (tcx , starting_item , state , & mut recursion_depths , recursion_limit , CollectionMode :: UsedItems ,) ; }
};
}
