// Generated macro for DropTree (struct)
macro_rules! Depcrate_builder_scopeDropTree {
() => {
// Module: crate::builder::scope
// Provides: {"DropTree"}
// Dependencies: {}
# [doc = " A tree of drops that we have deferred lowering. It's used for:"] # [doc = ""] # [doc = " * Drops on unwind paths"] # [doc = " * Drops on coroutine drop paths (when a suspended coroutine is dropped)"] # [doc = " * Drops on return and loop exit paths"] # [doc = " * Drops on the else path in an `if let` chain"] # [doc = ""] # [doc = " Once no more nodes could be added to the tree, we lower it to MIR in one go"] # [doc = " in `build_mir`."] # [derive (Debug)] struct DropTree { # [doc = " Nodes in the drop tree, containing drop data and a link to the next node."] drop_nodes : IndexVec < DropIdx , DropNode > , # [doc = " Map for finding the index of an existing node, given its contents."] existing_drops_map : FxHashMap < DropNodeKey , DropIdx > , # [doc = " Edges into the `DropTree` that need to be added once it's lowered."] entry_points : Vec < (DropIdx , BasicBlock) > , }
};
}
