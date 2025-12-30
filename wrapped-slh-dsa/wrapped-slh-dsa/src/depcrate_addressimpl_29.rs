// Generated macro for impl_29 (impl)
macro_rules! Depcrate_addressimpl_29 {
() => {
// Module: crate::address
// Provides: {"impl_29"}
// Dependencies: {}
impl WotsHash { pub (crate) fn prf_adrs (& self) -> WotsPrf { WotsPrf { layer_adrs : self . layer_adrs , tree_adrs_low : self . tree_adrs_low , tree_adrs_high : self . tree_adrs_high , type_const : WotsPrf :: TYPE_CONST . into () , key_pair_adrs : self . key_pair_adrs , chain_adrs : 0 . into () , hash_adrs : 0 . into () , } } pub (crate) fn pk_adrs (& self) -> WotsPk { WotsPk { layer_adrs : self . layer_adrs , tree_adrs_low : self . tree_adrs_low , tree_adrs_high : self . tree_adrs_high , type_const : WotsPk :: TYPE_CONST . into () , key_pair_adrs : self . key_pair_adrs , padding : 0 . into () , } } pub (crate) fn tree_adrs (& self) -> HashTree { HashTree { layer_adrs : self . layer_adrs , tree_adrs_low : self . tree_adrs_low , tree_adrs_high : self . tree_adrs_high , type_const : HashTree :: TYPE_CONST . into () , padding : 0 . into () , tree_height : 0 . into () , tree_index : 0 . into () , } } }
};
}
