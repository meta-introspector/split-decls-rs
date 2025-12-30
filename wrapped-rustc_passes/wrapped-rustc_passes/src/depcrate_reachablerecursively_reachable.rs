// Generated macro for recursively_reachable (function)
macro_rules! Depcrate_reachablerecursively_reachable {
() => {
// Module: crate::reachable
// Provides: {"recursively_reachable"}
// Dependencies: {}
# [doc = " Determines whether this item is recursive for reachability. See `is_recursively_reachable_local`"] # [doc = " below for details."] fn recursively_reachable (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { tcx . generics_of (def_id) . requires_monomorphization (tcx) || tcx . cross_crate_inlinable (def_id) || tcx . is_const_fn (def_id) }
};
}
