// Generated macro for create_assigned_tuple_shuffle_call_fmt (function)
macro_rules! Depcrate_big_endiancreate_assigned_tuple_shuffle_call_fmt {
() => {
// Module: crate::big_endian
// Provides: {"create_assigned_tuple_shuffle_call_fmt"}
// Dependencies: {}
fn create_assigned_tuple_shuffle_call_fmt (variable_name : & String , idx : u32 , array_lanes : & String ,) -> String { format ! ("{variable_name}.{idx} = unsafe {{ simd_shuffle!({variable_name}.{idx}, {variable_name}.{idx}, {array_lanes}) }};\n") }
};
}
