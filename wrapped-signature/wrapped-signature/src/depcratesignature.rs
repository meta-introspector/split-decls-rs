// Generated macro for Signature (struct)
macro_rules! DepcrateSignature {
() => {
// Module: crate
// Provides: {"Signature"}
// Dependencies: {}
# [repr (transparent)] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [derive (Clone , Copy , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "bytemuck" , derive (bytemuck_derive :: Pod , bytemuck_derive :: Zeroable))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [cfg_attr (feature = "wincode" , derive (SchemaWrite , SchemaRead))] pub struct Signature (# [cfg_attr (feature = "serde" , serde (with = "BigArray"))] [u8 ; SIGNATURE_BYTES] ,) ;
};
}
