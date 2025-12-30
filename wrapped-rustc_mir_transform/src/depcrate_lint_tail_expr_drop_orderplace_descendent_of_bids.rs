// Generated macro for place_descendent_of_bids (function)
macro_rules! Depcrate_lint_tail_expr_drop_orderplace_descendent_of_bids {
() => {
// Module: crate::lint_tail_expr_drop_order
// Provides: {"place_descendent_of_bids"}
// Dependencies: {}
# [doc = " Check if a moved place at `idx` is a part of a BID."] # [doc = " The use of this check is that we will consider drops on these"] # [doc = " as a drop of the overall BID and, thus, we can exclude it from the diagnosis."] fn place_descendent_of_bids < 'tcx > (mut idx : MovePathIndex , move_data : & MoveData < 'tcx > , bids : & UnordSet < & Place < 'tcx > > ,) -> bool { loop { let path = & move_data . move_paths [idx] ; if bids . contains (& path . place) { return true ; } if let Some (parent) = path . parent { idx = parent ; } else { return false ; } } }
};
}
