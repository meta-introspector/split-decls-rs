// Generated macro for create_shuffle_call_fmt (function)
macro_rules! Depcrate_big_endiancreate_shuffle_call_fmt {
() => {
// Module: crate::big_endian
// Provides: {"create_shuffle_call_fmt"}
// Dependencies: {}
fn create_shuffle_call_fmt (variable_name : & String , _type_kind : & TypeKind , array_lanes : & String ,) -> String { format ! ("simd_shuffle!({variable_name}, {variable_name}, {array_lanes})") }
};
}
