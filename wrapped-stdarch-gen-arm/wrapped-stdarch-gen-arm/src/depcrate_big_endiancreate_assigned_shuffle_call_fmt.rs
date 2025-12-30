// Generated macro for create_assigned_shuffle_call_fmt (function)
macro_rules! Depcrate_big_endiancreate_assigned_shuffle_call_fmt {
() => {
// Module: crate::big_endian
// Provides: {"create_assigned_shuffle_call_fmt"}
// Dependencies: {}
fn create_assigned_shuffle_call_fmt (variable_name : & String , type_kind : & TypeKind , array_lanes : & String ,) -> String { format ! ("let {variable_name}: {type_kind} = unsafe {{ simd_shuffle!({variable_name}, {variable_name}, {array_lanes}) }}") }
};
}
