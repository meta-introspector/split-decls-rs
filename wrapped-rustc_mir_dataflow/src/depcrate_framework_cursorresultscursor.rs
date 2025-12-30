// Generated macro for ResultsCursor (struct)
macro_rules! Depcrate_framework_cursorResultsCursor {
() => {
// Module: crate::framework::cursor
// Provides: {"ResultsCursor"}
// Dependencies: {}
# [doc = " Allows random access inspection of the results of a dataflow analysis. Use this when you want"] # [doc = " to inspect domain values only in certain locations; use `ResultsVisitor` if you want to inspect"] # [doc = " domain values in many or all locations."] # [doc = ""] # [doc = " Because `Results` only has domain values for the entry of each basic block, these inspections"] # [doc = " involve some amount of domain value recomputations. This cursor only has linear performance"] # [doc = " within a basic block when its statements are visited in the same order as the `DIRECTION` of"] # [doc = " the analysis. In the worst case—when statements are visited in *reverse* order—performance will"] # [doc = " be quadratic in the number of statements in the block. The order in which basic blocks are"] # [doc = " inspected has no impact on performance."] pub struct ResultsCursor < 'mir , 'tcx , A > where A : Analysis < 'tcx > , { body : & 'mir mir :: Body < 'tcx > , analysis : CowMut < 'mir , A > , results : Cow < 'mir , Results < A :: Domain > > , state : A :: Domain , pos : CursorPosition , # [doc = " Indicates that `state` has been modified with a custom effect."] # [doc = ""] # [doc = " When this flag is set, we need to reset to an entry set before doing a seek."] state_needs_reset : bool , # [cfg (debug_assertions)] reachable_blocks : DenseBitSet < BasicBlock > , }
};
}
