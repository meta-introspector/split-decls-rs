// Generated macro for WotsPrf (struct)
macro_rules! Depcrate_addressWotsPrf {
() => {
// Module: crate::address
// Provides: {"WotsPrf"}
// Dependencies: {}
# [derive (Clone , IntoBytes , Immutable)] # [repr (C)] pub (crate) struct WotsPrf { pub (crate) layer_adrs : U32 , pub (crate) tree_adrs_high : U32 , pub (crate) tree_adrs_low : U64 , type_const : U32 , pub (crate) key_pair_adrs : U32 , pub (crate) chain_adrs : U32 , hash_adrs : U32 , }
};
}
