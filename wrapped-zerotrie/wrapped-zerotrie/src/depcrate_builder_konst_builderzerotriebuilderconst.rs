// Generated macro for ZeroTrieBuilderConst (struct)
macro_rules! Depcrate_builder_konst_builderZeroTrieBuilderConst {
() => {
// Module: crate::builder::konst::builder
// Provides: {"ZeroTrieBuilderConst"}
// Dependencies: {}
# [doc = " A low-level builder for ZeroTrieSimpleAscii. Works in const contexts."] # [doc = ""] # [doc = " All methods that grow the trie will panic if the capacity N is not enough."] pub (crate) struct ZeroTrieBuilderConst < const N : usize > { data : ConstArrayBuilder < N , u8 > , }
};
}
