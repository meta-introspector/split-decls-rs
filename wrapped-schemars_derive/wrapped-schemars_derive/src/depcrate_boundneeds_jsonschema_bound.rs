// Generated macro for needs_jsonschema_bound (function)
macro_rules! Depcrate_boundneeds_jsonschema_bound {
() => {
// Module: crate::bound
// Provides: {"needs_jsonschema_bound"}
// Dependencies: {}
fn needs_jsonschema_bound (field : & Field , variant : Option < & Variant >) -> bool { if let Some (variant) = variant { if variant . serde_attrs . skip_deserializing () && variant . serde_attrs . skip_serializing () { return false ; } } if field . serde_attrs . skip_deserializing () && field . serde_attrs . skip_serializing () { return false ; } true }
};
}
