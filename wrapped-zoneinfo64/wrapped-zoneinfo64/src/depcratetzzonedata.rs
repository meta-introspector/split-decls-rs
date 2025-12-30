// Generated macro for TzZoneData (struct)
macro_rules! DepcrateTzZoneData {
() => {
// Module: crate
// Provides: {"TzZoneData"}
// Dependencies: {}
# [derive (Clone)] struct TzZoneData < 'a > { # [doc = " Transitions before the epoch of i32::MIN"] trans_pre32 : & 'a [(i32 , i32)] , # [doc = " Transitions with epoch values that can fit in an i32"] trans : & 'a [i32] , # [doc = " Transitions after the epoch of i32::MAX"] trans_post32 : & 'a [(i32 , i32)] , # [doc = " Map to offset from transitions. Treat [trans_pre32, trans, trans_post32]"] # [doc = " as a single array and use its corresponding index into this to get the index"] # [doc = " in type_offsets. The index in type_offsets is the *new* offset after the"] # [doc = " matching transition"] type_map : & 'a [u8] , # [doc = " Offsets. First entry is standard time, second entry is offset from standard time (if any)"] type_offsets : & 'a [(i32 , i32)] , # [doc = " An index into the Rules table,"] # [doc = " its standard_offset_seconds, and its starting year."] final_rule_offset_year : Option < (u32 , i32 , i32) > , # [allow (dead_code)] links : & 'a [u32] , }
};
}
