// Generated macro for propagate (function)
macro_rules! Depcrate_proppropagate {
() => {
// Module: crate::prop
// Provides: {"propagate"}
// Dependencies: {}
# [doc = " Propagate enqueued assignments."] # [doc = ""] # [doc = " Returns when all enqueued assignments are propagated, including newly propagated assignemnts, or"] # [doc = " if there is a conflict."] # [doc = ""] # [doc = " On conflict the first propagation that would assign the opposite value to an already assigned"] # [doc = " literal is returned."] pub fn propagate (mut ctx : partial ! (Context , mut AssignmentP , mut ClauseAllocP , mut ImplGraphP , mut TrailP , mut WatchlistsP , BinaryClausesP , ClauseDbP ,) ,) -> Result < () , Conflict > { enable_watchlists (ctx . borrow ()) ; while let Some (lit) = ctx . part_mut (TrailP) . pop_queue () { binary :: propagate_binary (ctx . borrow () , lit) ? ; long :: propagate_long (ctx . borrow () , lit) ? ; } Ok (()) }
};
}
