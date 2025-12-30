// Generated macro for DropsReachable (struct)
macro_rules! Depcrate_lint_tail_expr_drop_orderDropsReachable {
() => {
// Module: crate::lint_tail_expr_drop_order
// Provides: {"DropsReachable"}
// Dependencies: {}
struct DropsReachable < 'a , 'mir , 'tcx > { body : & 'a Body < 'tcx > , place : & 'a Place < 'tcx > , drop_span : & 'a mut Option < Span > , move_data : & 'a MoveData < 'tcx > , maybe_init : & 'a mut ResultsCursor < 'mir , 'tcx , MaybeInitializedPlaces < 'mir , 'tcx > > , block_drop_value_info : & 'a mut IndexSlice < BasicBlock , MovePathIndexAtBlock > , collected_drops : & 'a mut MixedBitSet < MovePathIndex > , visited : FxHashMap < BasicBlock , Rc < RefCell < MixedBitSet < MovePathIndex > > > > , }
};
}
