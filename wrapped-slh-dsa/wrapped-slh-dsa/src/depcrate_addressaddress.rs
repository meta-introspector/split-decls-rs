// Generated macro for Address (trait)
macro_rules! Depcrate_addressAddress {
() => {
// Module: crate::address
// Provides: {"Address"}
// Dependencies: {}
# [doc = " `Address` represents a hash address as defined by FIPS-205 section 4.2"] pub (crate) trait Address : AsRef < [u8] > { const TYPE_CONST : u32 ; # [allow (clippy :: doc_markdown)] # [doc = " Returns the address as a compressed 22-byte array"] # [doc = " ADRSc = ADRS[3] ∥ ADRS[8 : 16] ∥ ADRS[19] ∥ ADRS[20 : 32]"] fn compressed (& self) -> Array < u8 , U22 > { let bytes = self . as_ref () ; let mut compressed = Array :: < u8 , U22 > :: default () ; compressed [0] = bytes [3] ; compressed [1 .. 9] . copy_from_slice (& bytes [8 .. 16]) ; compressed [9] = bytes [19] ; compressed [10 .. 22] . copy_from_slice (& bytes [20 .. 32]) ; compressed } }
};
}
