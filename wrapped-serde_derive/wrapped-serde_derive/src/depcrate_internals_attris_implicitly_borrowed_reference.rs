// Generated macro for is_implicitly_borrowed_reference (function)
macro_rules! Depcrate_internals_attris_implicitly_borrowed_reference {
() => {
// Module: crate::internals::attr
// Provides: {"is_implicitly_borrowed_reference"}
// Dependencies: {}
fn is_implicitly_borrowed_reference (ty : & syn :: Type) -> bool { is_reference (ty , is_str) || is_reference (ty , is_slice_u8) }
};
}
