// Generated macro for HashTree (struct)
macro_rules! Depcrate_addressHashTree {
() => {
// Module: crate::address
// Provides: {"HashTree"}
// Dependencies: {}
# [derive (Clone , IntoBytes , Immutable)] # [repr (C)] pub (crate) struct HashTree { pub (crate) layer_adrs : U32 , pub (crate) tree_adrs_high : U32 , pub (crate) tree_adrs_low : U64 , type_const : U32 , padding : U32 , pub (crate) tree_height : U32 , pub (crate) tree_index : U32 , }
};
}
