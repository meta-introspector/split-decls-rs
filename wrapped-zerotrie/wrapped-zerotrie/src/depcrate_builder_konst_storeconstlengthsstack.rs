// Generated macro for ConstLengthsStack (struct)
macro_rules! Depcrate_builder_konst_storeConstLengthsStack {
() => {
// Module: crate::builder::konst::store
// Provides: {"ConstLengthsStack"}
// Dependencies: {}
# [doc = " A data structure that holds up to K [`BranchMeta`] items."] # [doc = ""] # [doc = " Note: It should be possible to store the required data in the builder buffer itself,"] # [doc = " which would eliminate the need for this helper struct and the limit it imposes."] pub (crate) struct ConstLengthsStack < const K : usize > { data : [Option < BranchMeta > ; K] , idx : usize , }
};
}
