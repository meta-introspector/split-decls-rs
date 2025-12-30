// Generated macro for impl_397 (impl)
macro_rules! Depcrate_transformimpl_397 {
() => {
// Module: crate::transform
// Provides: {"impl_397"}
// Dependencies: {}
impl Transform for RemoveRefSiblings { fn transform (& mut self , schema : & mut Schema) { transform_subschemas (self , schema) ; if let Some (obj) = schema . as_object_mut () . filter (| o | o . len () > 1) { if let Some (ref_value) = obj . remove ("$ref") { if let Value :: Array (all_of) = obj . entry ("allOf") . or_insert (Value :: Array (Vec :: new ())) { all_of . push (json ! ({ "$ref" : ref_value })) ; } } } } }
};
}
