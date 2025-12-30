// Generated macro for type_has_tuple (function)
macro_rules! Depcrate_big_endiantype_has_tuple {
() => {
// Module: crate::big_endian
// Provides: {"type_has_tuple"}
// Dependencies: {}
pub fn type_has_tuple (type_kind : & TypeKind) -> bool { if let TypeKind :: Vector (vector_type) = type_kind { vector_type . tuple_size () . is_some () } else { false } }
};
}
