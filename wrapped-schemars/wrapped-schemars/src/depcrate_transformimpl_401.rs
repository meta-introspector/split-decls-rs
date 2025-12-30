// Generated macro for impl_401 (impl)
macro_rules! Depcrate_transformimpl_401 {
() => {
// Module: crate::transform
// Provides: {"impl_401"}
// Dependencies: {}
impl Transform for ReplaceConstValue { fn transform (& mut self , schema : & mut Schema) { transform_subschemas (self , schema) ; if let Some (value) = schema . remove ("const") { schema . insert ("enum" . into () , Value :: Array (vec ! [value])) ; } } }
};
}
