// Generated macro for Secp256k1Pubkey (struct)
macro_rules! DepcrateSecp256k1Pubkey {
() => {
// Module: crate
// Provides: {"Secp256k1Pubkey"}
// Dependencies: {}
# [repr (transparent)] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "borsh" , derive (BorshSerialize , BorshDeserialize , BorshSchema) , borsh (crate = "borsh"))] # [derive (Clone , Copy , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct Secp256k1Pubkey (pub [u8 ; SECP256K1_PUBLIC_KEY_LENGTH]) ;
};
}
