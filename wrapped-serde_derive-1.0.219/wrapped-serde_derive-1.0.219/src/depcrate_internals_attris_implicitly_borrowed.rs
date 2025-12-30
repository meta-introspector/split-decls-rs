// Generated macro for is_implicitly_borrowed (function)
macro_rules! Depcrate_internals_attris_implicitly_borrowed {
() => {
// Module: crate::internals::attr
// Provides: {"is_implicitly_borrowed"}
// Dependencies: {}
fn is_implicitly_borrowed (ty : & syn :: Type) -> bool { is_implicitly_borrowed_reference (ty) || is_option (ty , is_implicitly_borrowed_reference) }
};
}
