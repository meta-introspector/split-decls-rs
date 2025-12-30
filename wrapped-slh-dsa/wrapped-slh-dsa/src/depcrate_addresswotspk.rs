// Generated macro for WotsPk (struct)
macro_rules! Depcrate_addressWotsPk {
() => {
// Module: crate::address
// Provides: {"WotsPk"}
// Dependencies: {}
# [derive (Clone , IntoBytes , Immutable)] # [repr (C)] pub (crate) struct WotsPk { pub (crate) layer_adrs : U32 , pub (crate) tree_adrs_high : U32 , pub (crate) tree_adrs_low : U64 , type_const : U32 , pub (crate) key_pair_adrs : U32 , padding : U64 , }
};
}
