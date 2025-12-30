// Generated macro for is_primitive_type (function)
macro_rules! Depcrate_internals_attris_primitive_type {
() => {
// Module: crate::internals::attr
// Provides: {"is_primitive_type"}
// Dependencies: {}
fn is_primitive_type (ty : & syn :: Type , primitive : & str) -> bool { match ungroup (ty) { syn :: Type :: Path (ty) => ty . qself . is_none () && is_primitive_path (& ty . path , primitive) , _ => false , } }
};
}
