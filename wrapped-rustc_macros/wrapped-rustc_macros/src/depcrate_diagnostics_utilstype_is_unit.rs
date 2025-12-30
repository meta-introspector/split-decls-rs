// Generated macro for type_is_unit (function)
macro_rules! Depcrate_diagnostics_utilstype_is_unit {
() => {
// Module: crate::diagnostics::utils
// Provides: {"type_is_unit"}
// Dependencies: {}
# [doc = " Checks whether the type `ty` is `()`."] pub (crate) fn type_is_unit (ty : & Type) -> bool { if let Type :: Tuple (TypeTuple { elems , .. }) = ty { elems . is_empty () } else { false } }
};
}
