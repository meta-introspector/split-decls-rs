// Generated macro for is_slice_u8 (function)
macro_rules! Depcrate_internals_attris_slice_u8 {
() => {
// Module: crate::internals::attr
// Provides: {"is_slice_u8"}
// Dependencies: {}
fn is_slice_u8 (ty : & syn :: Type) -> bool { match ungroup (ty) { syn :: Type :: Slice (ty) => is_primitive_type (& ty . elem , "u8") , _ => false , } }
};
}
