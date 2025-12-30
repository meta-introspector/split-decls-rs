// Generated macro for allow_null (function)
macro_rules! Depcrate__privateallow_null {
() => {
// Module: crate::_private
// Provides: {"allow_null"}
// Dependencies: {}
pub (crate) fn allow_null (generator : & mut SchemaGenerator , schema : & mut Schema) { fn is_null_schema (value : & Value) -> bool { < & Schema > :: try_from (value) . is_ok_and (| s | s . has_type ("null")) } match schema . try_as_object_mut () { Ok (obj) => { if obj . len () == 1 && obj . get ("anyOf") . and_then (Value :: as_array) . is_some_and (| a | a . iter () . any (is_null_schema)) { return ; } if contains_immediate_subschema (obj) { * schema = json_schema ! ({ "anyOf" : [obj , < () >:: json_schema (generator)] }) ; return ; } if let Some (instance_type) = obj . get_mut ("type") { match instance_type { Value :: Array (array) => { let null = Value :: from ("null") ; if ! array . contains (& null) { array . push (null) ; } } Value :: String (string) => { if string != "null" { let current_type = core :: mem :: take (string) . into () ; * instance_type = Value :: Array (vec ! [current_type , "null" . into ()]) ; } } _ => { } } } if let Some (c) = obj . remove ("const") { if ! c . is_null () { obj . insert ("enum" . to_string () , Value :: Array (vec ! [c , Value :: Null])) ; } } else if let Some (Value :: Array (e)) = obj . get_mut ("enum") { if ! e . contains (& Value :: Null) { e . push (Value :: Null) ; } } } Err (true) => { } Err (false) => { * schema = < () > :: json_schema (generator) ; } } }
};
}
