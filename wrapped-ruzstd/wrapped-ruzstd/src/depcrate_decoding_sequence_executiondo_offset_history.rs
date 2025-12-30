// Generated macro for do_offset_history (function)
macro_rules! Depcrate_decoding_sequence_executiondo_offset_history {
() => {
// Module: crate::decoding::sequence_execution
// Provides: {"do_offset_history"}
// Dependencies: {}
# [doc = " Update the most recently used offsets to reflect the provided offset value, and return the"] # [doc = " \"actual\" offset needed because offsets are not stored in a raw way, some transformations are needed"] # [doc = " before you get a functional number."] fn do_offset_history (offset_value : u32 , lit_len : u32 , scratch : & mut [u32 ; 3]) -> u32 { let actual_offset = if lit_len > 0 { match offset_value { 1 ..= 3 => scratch [offset_value as usize - 1] , _ => { offset_value - 3 } } } else { match offset_value { 1 ..= 2 => scratch [offset_value as usize] , 3 => scratch [0] - 1 , _ => { offset_value - 3 } } } ; if lit_len > 0 { match offset_value { 1 => { } 2 => { scratch [1] = scratch [0] ; scratch [0] = actual_offset ; } _ => { scratch [2] = scratch [1] ; scratch [1] = scratch [0] ; scratch [0] = actual_offset ; } } } else { match offset_value { 1 => { scratch [1] = scratch [0] ; scratch [0] = actual_offset ; } 2 => { scratch [2] = scratch [1] ; scratch [1] = scratch [0] ; scratch [0] = actual_offset ; } _ => { scratch [2] = scratch [1] ; scratch [1] = scratch [0] ; scratch [0] = actual_offset ; } } } actual_offset }
};
}
