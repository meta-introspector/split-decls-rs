// Generated macro for impl_30 (impl)
macro_rules! Depcrate_addressimpl_30 {
() => {
// Module: crate::address
// Provides: {"impl_30"}
// Dependencies: {}
impl ForsTree { pub (crate) fn new (tree_adrs_low : u64 , key_pair_adrs : u32) -> ForsTree { ForsTree { layer_adrs : 0 . into () , tree_adrs_low : tree_adrs_low . into () , tree_adrs_high : 0 . into () , type_const : ForsTree :: TYPE_CONST . into () , key_pair_adrs : key_pair_adrs . into () , tree_height : 0 . into () , tree_index : 0 . into () , } } pub (crate) fn prf_adrs (& self) -> ForsPrf { ForsPrf { layer_adrs : 0 . into () , tree_adrs_low : self . tree_adrs_low , tree_adrs_high : self . tree_adrs_high , type_const : ForsPrf :: TYPE_CONST . into () , key_pair_adrs : self . key_pair_adrs , tree_height : 0 . into () , tree_index : self . tree_index , } } pub (crate) fn fors_roots (& self) -> ForsRoots { ForsRoots { layer_adrs : 0 . into () , tree_adrs_low : self . tree_adrs_low , tree_adrs_high : self . tree_adrs_high , type_const : ForsRoots :: TYPE_CONST . into () , key_pair_adrs : self . key_pair_adrs , padding : 0 . into () , } } }
};
}
