// Generated macro for BranchMeta (struct)
macro_rules! Depcrate_builder_branch_metaBranchMeta {
() => {
// Module: crate::builder::branch_meta
// Provides: {"BranchMeta"}
// Dependencies: {}
# [doc = " Intermediate metadata for a branch node under construction."] # [derive (Debug , Clone , Copy)] pub (crate) struct BranchMeta { # [doc = " The lead byte for this branch. Formerly it was required to be an ASCII byte, but now"] # [doc = " it can be any byte."] pub ascii : u8 , # [doc = " The size in bytes of the trie data reachable from this branch."] pub local_length : usize , # [doc = " The size in bytes of this and all later sibling branches."] pub cumulative_length : usize , # [doc = " The number of later sibling branches, including this."] pub count : usize , }
};
}
