// Generated macro for flatten (function)
macro_rules! Depcrate__privateflatten {
() => {
// Module: crate::_private
// Provides: {"flatten"}
// Dependencies: {}
pub fn flatten (schema : & mut Schema , other : Schema) { fn flatten_property (obj1 : & mut Map < String , Value > , key : String , value2 : Value) { match obj1 . entry (key) { Entry :: Vacant (vacant) => { vacant . insert (value2) ; } Entry :: Occupied (occupied) => { match occupied . key () . as_str () { "required" | "allOf" => { if let Value :: Array (a1) = occupied . into_mut () { if let Value :: Array (a2) = value2 { a1 . extend (a2) ; } } } "properties" | "patternProperties" => { if let Value :: Object (o1) = occupied . into_mut () { if let Value :: Object (o2) = value2 { o1 . extend (o2) ; } } } "oneOf" | "anyOf" => { let (key , current) = occupied . remove_entry () ; flatten_property (obj1 , "allOf" . to_owned () , json ! ([{ & key : current } , { key : value2 }]) ,) ; } _ => { } } } } } match other . try_to_object () { Err (false) => { } Err (true) => { if let Some (obj) = schema . as_object_mut () { if ! obj . contains_key ("additionalProperties") && ! obj . contains_key ("unevaluatedProperties") { let key = if contains_immediate_subschema (obj) { "unevaluatedProperties" } else { "additionalProperties" } ; obj . insert (key . to_owned () , true . into ()) ; } } } Ok (mut obj2) => { let obj1 = schema . ensure_object () ; normalise_additional_unevaluated_properties (obj1 , & obj2) ; normalise_additional_unevaluated_properties (& mut obj2 , obj1) ; for (key , value2) in obj2 { flatten_property (obj1 , key , value2) ; } } } }
};
}
