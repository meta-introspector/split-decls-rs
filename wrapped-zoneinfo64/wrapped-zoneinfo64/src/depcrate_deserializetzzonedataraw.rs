// Generated macro for TzZoneDataRaw (struct)
macro_rules! Depcrate_deserializeTzZoneDataRaw {
() => {
// Module: crate::deserialize
// Provides: {"TzZoneDataRaw"}
// Dependencies: {}
# [derive (Debug , Deserialize)] # [serde (rename_all = "camelCase")] pub struct TzZoneDataRaw < 'a > { # [serde (borrow , deserialize_with = "resb::binary::helpers::i32_tuple")] type_offsets : & 'a [(i32 , i32)] , # [serde (borrow , default , deserialize_with = "resb::binary::helpers::option_i32")] trans : Option < & 'a [i32] > , # [serde (borrow , default , deserialize_with = "resb::binary::helpers::option_i32_tuple")] trans_pre32 : Option < & 'a [(i32 , i32)] > , # [serde (borrow , default , deserialize_with = "resb::binary::helpers::option_i32_tuple")] trans_post32 : Option < & 'a [(i32 , i32)] > , type_map : Option < & 'a [u8] > , # [serde (borrow , default , deserialize_with = "resb::binary::helpers::option_utf_16")] final_rule : Option < & 'a PotentialUtf16 > , final_raw : Option < i32 > , final_year : Option < i32 > , # [allow (dead_code)] # [serde (borrow , default , deserialize_with = "resb::binary::helpers::option_u32")] links : Option < & 'a [u32] > , }
};
}
