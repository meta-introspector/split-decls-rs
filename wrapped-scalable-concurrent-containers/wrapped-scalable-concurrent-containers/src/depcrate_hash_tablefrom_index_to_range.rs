// Generated macro for from_index_to_range (function)
macro_rules! Depcrate_hash_tablefrom_index_to_range {
() => {
// Module: crate::hash_table
// Provides: {"from_index_to_range"}
// Dependencies: {}
# [doc = " For the given index in the current array, calculate the respective range in the old array."] # [inline] const fn from_index_to_range (from_len : usize , to_len : usize , from_index : usize) -> (usize , usize) { debug_assert ! (from_len . is_power_of_two () && to_len . is_power_of_two ()) ; if from_len < to_len { let ratio = to_len / from_len ; let start_index = from_index * ratio ; debug_assert ! (start_index + ratio <= to_len ,) ; (start_index , start_index + ratio) } else { let ratio = from_len / to_len ; let start_index = from_index / ratio ; debug_assert ! (start_index < to_len ,) ; (start_index , start_index + 1) } }
};
}
