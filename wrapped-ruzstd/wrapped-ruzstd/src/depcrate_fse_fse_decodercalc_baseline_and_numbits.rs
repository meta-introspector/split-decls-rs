// Generated macro for calc_baseline_and_numbits (function)
macro_rules! Depcrate_fse_fse_decodercalc_baseline_and_numbits {
() => {
// Module: crate::fse::fse_decoder
// Provides: {"calc_baseline_and_numbits"}
// Dependencies: {}
fn calc_baseline_and_numbits (num_states_total : u32 , num_states_symbol : u32 , state_number : u32 ,) -> (u32 , u8) { if num_states_symbol == 0 { return (0 , 0) ; } let num_state_slices = if 1 << (highest_bit_set (num_states_symbol) - 1) == num_states_symbol { num_states_symbol } else { 1 << (highest_bit_set (num_states_symbol)) } ; let num_double_width_state_slices = num_state_slices - num_states_symbol ; let num_single_width_state_slices = num_states_symbol - num_double_width_state_slices ; let slice_width = num_states_total / num_state_slices ; let num_bits = highest_bit_set (slice_width) - 1 ; if state_number < num_double_width_state_slices { let baseline = num_single_width_state_slices * slice_width + state_number * slice_width * 2 ; (baseline , num_bits as u8 + 1) } else { let index_shifted = state_number - num_double_width_state_slices ; ((index_shifted * slice_width) , num_bits as u8) } }
};
}
