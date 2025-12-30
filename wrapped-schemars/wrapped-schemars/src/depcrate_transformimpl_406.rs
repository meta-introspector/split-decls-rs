// Generated macro for impl_406 (impl)
macro_rules! Depcrate_transformimpl_406 {
() => {
// Module: crate::transform
// Provides: {"impl_406"}
// Dependencies: {}
impl Transform for AddNullable { fn transform (& mut self , schema : & mut Schema) { if schema . has_type ("null") { schema . insert ("nullable" . into () , true . into ()) ; let ty = schema . get_mut ("type") . unwrap () ; let only_allows_null = ty . is_string () || ty . as_array () . unwrap () . iter () . all (| v | v == "null") ; if only_allows_null { if self . add_const_null { schema . insert ("const" . to_string () , Value :: Null) ; if self . remove_null_type { schema . remove ("type") ; } } else if self . remove_null_type { * ty = Value :: Array (Vec :: new ()) ; } } else if self . remove_null_type { let array = ty . as_array_mut () . unwrap () ; array . retain (| t | t != "null") ; if array . len () == 1 { * ty = array . remove (0) ; } } } transform_subschemas (self , schema) ; } }
};
}
