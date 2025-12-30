// Generated macro for is_primitive_path (function)
macro_rules! Depcrate_internals_attris_primitive_path {
() => {
// Module: crate::internals::attr
// Provides: {"is_primitive_path"}
// Dependencies: {}
fn is_primitive_path (path : & syn :: Path , primitive : & str) -> bool { path . leading_colon . is_none () && path . segments . len () == 1 && path . segments [0] . ident == primitive && path . segments [0] . arguments . is_empty () }
};
}
