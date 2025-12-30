// Generated macro for create_shuffle_call (function)
macro_rules! Depcrate_big_endiancreate_shuffle_call {
() => {
// Module: crate::big_endian
// Provides: {"create_shuffle_call"}
// Dependencies: {}
# [doc = " Create a `simd_shuffle!(<...>, [...])` call"] pub fn create_shuffle_call (variable_name : & String , type_kind : & TypeKind) -> Option < Expression > { create_shuffle_internal (variable_name , type_kind , create_assigned_tuple_shuffle_call_fmt , create_shuffle_call_fmt ,) }
};
}
