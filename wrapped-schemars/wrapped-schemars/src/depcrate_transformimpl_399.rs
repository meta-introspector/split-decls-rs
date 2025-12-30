// Generated macro for impl_399 (impl)
macro_rules! Depcrate_transformimpl_399 {
() => {
// Module: crate::transform
// Provides: {"impl_399"}
// Dependencies: {}
impl Transform for SetSingleExample { fn transform (& mut self , schema : & mut Schema) { transform_subschemas (self , schema) ; if let Some (Value :: Array (examples)) = schema . remove ("examples") { if let Some (first_example) = examples . into_iter () . next () { schema . insert ("example" . into () , first_example) ; } } } }
};
}
